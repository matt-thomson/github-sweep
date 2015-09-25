use std::env::VarError;
use std::fmt::{Display, Formatter, Result};

pub enum Error {
    EnvError { var: &'static str, err: VarError }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Error::EnvError { ref var, ref err } =>
                write!(f, "Error reading environment variable {}: {}", var, err)
        }
    }
}
