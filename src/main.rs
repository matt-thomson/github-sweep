extern crate hyper;

mod credentials;
mod error;
mod http;

use hyper::client::Response;

use std::process;

use http::Client;
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
    let client = Client::new(credentials);

    let response = try!(client.put("https://api.github.com/notifications", "{}"));
    check(&response)
}

fn check(response: &Response) -> Result<(), Error> {
    if response.status.is_success() {
        Ok(())
    } else {
        Err(Error::Request { status: response.status })
    }
}
