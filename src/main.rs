extern crate hyper;

mod credentials;
mod error;
mod http;

use std::process;
use std::thread;

use credentials::Credentials;
use http::Client;
use error::Error;

fn main() {
    let credentials = credentials();
    let client = Client::new(credentials);

    loop {
        let result = run(&client);

        if result.is_err() {
            println!("{}", result.unwrap_err());
        }

        thread::sleep_ms(5000);
    }
}

fn credentials() -> Credentials {
    match credentials::from_env() {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn run(client: &Client) -> Result<(), Error> {
    let response = try!(client.put("https://api.github.com/notifications", "{}"));

    if response.status.is_success() {
        Ok(())
    } else {
        Err(Error::Request { status: response.status })
    }
}
