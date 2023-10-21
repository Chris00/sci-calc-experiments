// https://docs.github.com/en/actions/security-guides/automatic-token-authentication

use std::{env, error::Error};
use octocrab::{self, params};

fn get_env(env: impl AsRef<str>) -> Option<String> {
    let env = env.as_ref();
    env::vars().find_map(|(k, v)| {
        if k == env { Some(v) } else { None }})
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>  {
    for (k, v) in env::vars() {
        println!("{k} → {v}");
    }

    let octocrab = octocrab::instance();
    let repo = get_env("GITHUB_REPOSITORY")
        .ok_or("Github repository".to_string())?;
    let mut repo = repo.split("/");
    let name = repo.next().ok_or("Repository owner")?;
    let repo = repo.next().ok_or("Repository name")?;
    let pulls = octocrab.pulls(name, repo)
        .list()
        .state(params::State::Open)
        .send()
        .await?;

    for p in pulls {
        println!("- URL: {}", p.url)
    }

    if let Some(event) = get_env("GITHUB_EVENT_NAME") {
        if event == "pull_request" {
            println!("This is a PR");
        } else {
            println!("Event: {event:?}");
        }
    }
    Ok(())
}
