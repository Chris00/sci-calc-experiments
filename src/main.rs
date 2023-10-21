// https://docs.github.com/en/actions/security-guides/automatic-token-authentication

use std::env;

fn main() {
    for (k, v) in env::vars() {
        println!("{k} → {v}");
    }

    let event = env::vars().find_map(|(k, v)| {
        if k == "GITHUB_EVENT_NAME" { Some(v) } else { None }});
    if let Some(event) = event {
        if event == "push" {
            println!("This is a push");
        } else {
            println!("Event: {event:?}");
        }
    }
}
