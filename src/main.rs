extern crate hyper;

use std::{env, process};

use hyper::Client;
use hyper::header::{Authorization, Basic, UserAgent};

fn main() {
    let user = match env::var("GITHUB_SWEEP_USER") {
        Ok(val) => val,
        Err(e) => {
            println!("Error reading GITHUB_SWEEP_USER environment variable: {}", e);
            process::exit(1)
        }
    };

    let token = match env::var("GITHUB_SWEEP_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            println!("Error reading GITHUB_SWEEP_TOKEN environment variable: {}", e);
            process::exit(1)
        }
    };

    let client = Client::new();

    let auth_header = Authorization(Basic { username: user, password: Some(token) });
    let user_agent_header = UserAgent("github-sweep".to_owned());

    let request = client
        .put("https://api.github.com/notifications")
        .header(auth_header)
        .header(user_agent_header)
        .body("{}");

    let response = match request.send() {
        Ok(val) => val,
        Err(e) => {
            println!("Error making request: {}", e);
            process::exit(1)
        }
    };

    println!("{:?}", response);
}
