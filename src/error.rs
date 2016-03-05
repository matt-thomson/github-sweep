use std::env::VarError;
use std::fmt::{Display, Formatter, Result};

use hyper;

pub enum Error {
    Env { var: &'static str, err: VarError },
    Http { url: &'static str, err: hyper::Error },
    Request { status: hyper::status::StatusCode }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Error::Env { ref var, ref err } =>
                write!(f, "Error reading environment variable {}: {}", var, err),
            Error::Http { ref url, ref err } =>
                write!(f, "Error making HTTP request to {}: {}", url, err),
            Error::Request { ref status } =>
                write!(f, "Invalid HTTP status ({})", status)
        }
    }
}
