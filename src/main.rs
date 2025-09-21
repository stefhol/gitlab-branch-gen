use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The Branch name to create
    branch_name: Option<String>,
    /// The List of repos to change
    #[arg(long)]
    repos: Option<Vec<String>>,
    /// The Api Key from gitlab
    #[arg(long)]
    gitlab_api: Option<String>,
    /// The gitlab instance to use
    #[arg(long)]
    gitlab_url: Option<String>,
    /// The file can contain gitlab_url, gitlab_api, and repos
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(branch_name) = cli.branch_name.as_deref() {
        println!("Value for branch_name: {branch_name}");
    }
    if let Some(gitlab_api) = cli.gitlab_api.as_deref() {
        println!("Value for gitlab_api: {gitlab_api}");
    }
    if let Some(gitlab_url) = cli.gitlab_url.as_deref() {
        println!("Value for gitlab_url: {gitlab_url}");
    }
    if let Some(repos) = cli.repos.as_deref() {
        println!("Value for repos: {}", repos.join(", "));
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    Ok(())
}
