use std::env;

fn main() {
    for (k, v) in env::vars() {
        println!("{k} → {v}");
    }
}
