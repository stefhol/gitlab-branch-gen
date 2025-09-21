use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{self},
    path::PathBuf,
};

use anyhow::{Ok, Result, anyhow};
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
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for repo in &self.repos {
            buffer += &format!("\n- {}", repo);
        }
        write!(
            f,
            "Branch to create: {}\nGitlab Instance: {}\nRepositories to change: {}",
            self.branch_name, self.gitlab_url, buffer
        )
    }
}
impl State {
    fn new(config: &ConfigFile, cli: &Cli) -> Result<Self> {
        let branch_name = cli
            .branch_name
            .clone()
            .ok_or(anyhow!("branch name has to be set"))?;
        let repos = config
            .repos
            .clone()
            .or(cli.repos.clone())
            .ok_or(anyhow!("There has to be atleast one repostiory set"))?;
        let gitlab_api = config
            .gitlab_api_key
            .clone()
            .or(cli.gitlab_api_key.clone())
            .ok_or(anyhow!("gitlab api key has to be set"))?;
        let gitlab_url = config
            .gitlab_url
            .clone()
            .or(cli.gitlab_url.clone())
            .or(Some(String::from("https://gitlab.com")))
            .unwrap();
        Ok(Self {
            branch_name,
            repos,
            gitlab_api,
            gitlab_url,
        })
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
    let state = State::new(&file, &cli)?;
    println!("{}", state);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", state.gitlab_api.parse()?);
    let _base_url = format!("{}/api/v4", state.gitlab_url);
    let _client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut repos: Vec<Repo> = vec![];
    println!("Gathering plan on creating branch {}", state.branch_name);
    for repo_name in state.repos {
        let repo_res = get_repository(&_client, &_base_url, &repo_name).await;
        if let Err(_err) = repo_res {
            println!("{repo_name}: Skipped Failure on retrieving information");
            repos.push(Repo {
                repo: None,
                skipped: true,
                repo_name,
            });
            continue;
        }
        let mut repo_res = repo_res.unwrap();
        repo_res.branch_to_create = Some(repo_name.to_string());
        let exists = branch_exists(&_client, &_base_url, &repo_res.id, &state.branch_name).await?;
        print!("{}", repo_res.web_url);
        if exists {
            print!(" ⚠️ Branch exists ⚠️")
        } else {
            print!(" ✅ ")
        }
        println!();
        repos.push(Repo {
            repo: Some(repo_res.clone()),
            skipped: exists,
            repo_name,
        });
    }

    println!("\n");

    println!("Execute? Type y to confirm");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    let buffer = buffer.trim();
    if buffer != "y" && buffer != "yes" {
        println!("Plan canceled");
        return Ok(());
    }

    println!("\n");

    for repo in repos {
        print!("{}: ", repo.repo_name);
        if repo.skipped || repo.repo.is_none() {
            println!("skipped");
            continue;
        }
        let repo = repo.repo.unwrap();

        print!("{} ", repo.web_url);

        let branch_created = create_branch(
            &_client,
            &_base_url,
            &repo.id,
            &state.branch_name,
            &repo.default_branch,
        )
        .await?;
        if branch_created {
            println!("created ✅");
        } else {
            println!("error ⛔");
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Repo {
    repo: Option<RepoResponse>,
    skipped: bool,
    repo_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RepoResponse {
    id: u64,
    default_branch: String,
    web_url: String,
    branch_to_create: Option<String>,
}

async fn get_repository(
    client: &reqwest::Client,
    base_url: &str,
    repo: &str,
) -> anyhow::Result<RepoResponse> {
    let repo = urlencoding::encode(repo);
    let response = client
        .get(format!("{base_url}/projects/{repo}"))
        .send()
        .await?;

    Ok(response.json().await?)
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
