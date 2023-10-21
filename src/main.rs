// https://docs.github.com/en/actions/security-guides/automatic-token-authentication

use std::{env, error::Error};
use octocrab::{self, Octocrab, params};

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
    let owner = repo.next().ok_or("Repository owner")?;
    let repo = repo.next().ok_or("Repository name")?;
    let pulls = octocrab.pulls(owner, repo)
        .list()
        .state(params::State::Open)
        .send()
        .await?;

    for p in pulls {
        println!("- URL: {}", p.url);
    }

    // To add a comment, one needs to be authenticated
    let token = std::env::var("GITHUB_TOKEN")?;
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let Some(number) = octocrab.pulls(owner, repo)
        .get(1).await?.comments
        else { Err("No comments")? };
    let body = format!(
        "Comment created from [Github action {}](https://github.com/Chris00/sci-calc-experiments/actions)",
        get_env("GITHUB_RUN_NUMBER").unwrap());
    println!("→ Want to add a comment to issue {number}.");
    let number = 1;
    println!("→ Want to add a comment to issue {number}.");
    let _c = octocrab.issues(owner, repo)
        .create_comment(number, body)
        .await?;


    if let Some(event) = get_env("GITHUB_EVENT_NAME") {
        if event == "pull_request" {
            println!("This is a PR");
        } else {
            println!("Event: {event:?}");
        }
    }
    Ok(())
}
