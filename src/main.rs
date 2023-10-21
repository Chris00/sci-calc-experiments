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
    let Some(comments) = octocrab.pulls(owner, repo)
        .get(1).await?.comments_url
        else { Err("No comments")? };
    let issue: u64 = {
        let mut segs = comments.path_segments().unwrap();
        for s in &mut segs {
            if s == "issues" { break }
        }
        segs.next().unwrap().parse()?
    };
    let user = get_env("GITHUB_ACTOR").unwrap_or(
        "Inconnu".to_string());
    let body = format!(
        "Cher {user},\n\n\
Ce commentaire a été créé par GH run #{n}, issue #{issue}",
        n = get_env("GITHUB_RUN_NUMBER").unwrap());
    println!("→ Want to add a comment to issue {issue}.");
    let _c = octocrab.issues(owner, repo)
        .create_comment(issue, body)
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
