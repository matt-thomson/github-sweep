use std::env::VarError;
use std::fmt::{Display, Formatter, Result};

use hyper;

pub enum Error {
    EnvError { var: &'static str, err: VarError },
    HttpError { url: &'static str, err: hyper::Error }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Error::EnvError { ref var, ref err } =>
                write!(f, "Error reading environment variable {}: {}", var, err),
            &Error::HttpError { ref url, ref err } =>
                write!(f, "Error making HTTP request to {}: {}", url, err)
        }
    }
}
