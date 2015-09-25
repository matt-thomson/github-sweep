use std::env;

use error::Error;

pub struct Credentials {
    pub username: String,
    pub token: String
}

pub fn from_env() -> Result<Credentials, Error> {
    let username = try!(env_var("GITHUB_SWEEP_USER"));
    let token = try!(env_var("GITHUB_SWEEP_TOKEN"));

    Ok(Credentials { username: username, token: token })
}

fn env_var(var: &'static str) -> Result<String, Error> {
    env::var(var).map_err(|err| Error::EnvError { var: var, err: err })
}
