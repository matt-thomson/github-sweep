use hyper;
use hyper::client::Response;
use hyper::header::{Authorization, Basic, Headers, UserAgent};

use credentials::Credentials;
use error::Error;

pub struct Client {
    client: hyper::Client,
    headers: Headers
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        let mut headers = Headers::new();

        headers.set(Authorization(Basic { username: credentials.username, password: Some(credentials.token) }));
        headers.set(UserAgent("github-sweep".to_owned()));

        Client { client: hyper::Client::new(), headers: headers }
    }

    pub fn put(&self, url: &'static str, body: &str) -> Result<Response, Error> {
        self.client
            .put(url)
            .headers(self.headers.clone())
            .body(body)
            .send()
            .map_err(|err| Error::HttpError { url: url, err: err })
    }
}
