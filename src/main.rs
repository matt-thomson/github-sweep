extern crate hyper;

mod credentials;
mod error;

use std::process;

use hyper::Client;
use hyper::header::{Authorization, Basic, UserAgent};

use error::Error;

fn main() {
    let result = run();

    if result.is_err() {
        println!("{}", result.unwrap_err());
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let credentials = try!(credentials::from_env());
    let client = Client::new();

    let auth_header = Authorization(Basic { username: credentials.username, password: Some(credentials.token) });
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
    Ok(())
}
