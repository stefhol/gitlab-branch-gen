use std::{
    collections::HashMap,
    fs,
    io::{self},
    path::PathBuf,
};

use anyhow::{Ok, Result};
use clap::Parser;
use os_path::OsPath;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    version,
    about = "Creates a defined branch in multiple gitlab repos. Config can be placed in `~/.config/branch-gen/config.yml`."
)]
struct Cli {
    /// The Branch name to create
    branch_name: Option<String>,
    /// The List of repos to change
    #[arg(long)]
    repos: Option<Vec<String>>,
    /// The Api Key from gitlab
    #[arg(long)]
    gitlab_api_key: Option<String>,
    /// The gitlab instance to use
    #[arg(long)]
    gitlab_url: Option<String>,

    /// If provided writes to the config
    #[arg(long, short)]
    update_config: bool,
    /// Overwrites the default config file location
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ConfigFile {
    repos: Option<Vec<String>>,
    gitlab_api_key: Option<String>,
    gitlab_url: Option<String>,
}

impl From<&Cli> for ConfigFile {
    fn from(value: &Cli) -> Self {
        Self {
            repos: value.repos.clone(),
            gitlab_api_key: value.gitlab_api_key.clone(),
            gitlab_url: value.gitlab_url.clone(),
        }
    }
}

struct State {
    branch_name: String,
    repos: Vec<String>,
    gitlab_api: String,
    gitlab_url: String,
}
impl State {
    fn new(config: &ConfigFile, cli: &Cli) -> Self {
        Self {
            branch_name: cli.branch_name.clone().expect("branch name has to be set"),
            repos: config
                .repos
                .clone()
                .or(cli.repos.clone())
                .expect("Repos has to be set"),
            gitlab_api: config
                .gitlab_api_key
                .clone()
                .or(cli.gitlab_api_key.clone())
                .expect("gitlab api key has to be set"),
            gitlab_url: config
                .gitlab_url
                .clone()
                .or(cli.gitlab_url.clone())
                .or(Some(String::from("https://gitlab.com")))
                .inspect(|v| println!("Using {v}"))
                .unwrap(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config_location = cli
        .config
        .as_deref()
        .map(|v| OsPath::from(v))
        .or(Some(OsPath::from("~/.config/gitlab-branch-gen/config.yml")))
        .unwrap();
    config_location.resolve();
    // Expands the "~" into the full home directory path
    let config_location =
        OsPath::from(shellexpand::tilde(&config_location.to_string()).to_string());

    fs::create_dir_all(config_location.parent().unwrap())?;
    if !config_location.exists() {
        fs::File::create(&config_location)?;
        let yaml = serde_yml::to_string(&ConfigFile::from(&cli))?;
        fs::write(&config_location, yaml)?;
        println!("Initialized Config file in {}", &config_location);
    }
    if cli.update_config {
        let yaml = serde_yml::to_string(&ConfigFile::from(&cli))?;
        fs::write(&config_location, yaml)?;
    }

    let file: ConfigFile = serde_yml::from_slice(&fs::read(&config_location)?)?;
    let state = State::new(&file, &cli);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", state.gitlab_api.parse()?);
    let _base_url = format!("{}/api/v4", state.gitlab_url);
    let _client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut repos = vec![];
    for repo in state.repos {
        let mut repo_res = get_repository(&_client, &_base_url, &repo).await?;
        repo_res.branch_to_create = Some(repo.to_string());
        repos.push(repo_res.clone());
        let exists = branch_exists(&_client, &_base_url, &repo_res.id, &state.branch_name).await?;
        println!("Plan on creating branch {}", state.branch_name);
        print!("{}", repo_res.web_url);
        if exists {
            print!(" ⚠️ Branch exists ⚠️")
        } else {
            print!(" ✅ Branch can be created ✅")
        }
        println!();
        println!("Execute? Type y to confirm");
    }
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    let buffer = buffer.trim();
    if buffer != "y" && buffer != "yes" {
        println!("Plan canceled");
        return Ok(());
    }

    for repo in repos {
        let branch_created = create_branch(
            &_client,
            &_base_url,
            &repo.id,
            &state.branch_name,
            &repo.default_branch,
        )
        .await?;
        print!("{}: ", repo.web_url);
        if branch_created {
            print!("created")
        } else {
            print!("skipped")
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repo {
    id: u64,
    default_branch: String,
    web_url: String,
    branch_to_create: Option<String>,
}

async fn get_repository(
    client: &reqwest::Client,
    base_url: &str,
    repo: &str,
) -> std::result::Result<Repo, reqwest::Error> {
    let repo = urlencoding::encode(repo);
    client
        .get(format!("{base_url}/projects/{repo}"))
        .send()
        .await?
        .json()
        .await
}

async fn create_branch(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &u64,
    branch_name: &str,
    reference: &str,
) -> std::result::Result<bool, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("branch", branch_name);
    map.insert("ref", reference);
    client
        .post(format!(
            "{base_url}/projects/{project_id}/repository/branches"
        ))
        .json(&map)
        .send()
        .await
        .map(|v| v.status() == 201)
}
async fn branch_exists(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &u64,
    branch_name: &str,
) -> std::result::Result<bool, reqwest::Error> {
    client
        .get(format!(
            "{base_url}/projects/{project_id}/repository/branches/{branch_name}"
        ))
        .send()
        .await
        .map(|resp| resp.status() == 200)
}
