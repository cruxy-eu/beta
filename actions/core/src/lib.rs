use std::env::{self, VarError};

use crate::io::{FromInputEnv, IoError, ToOutput};

pub mod io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("env var error: {0}")]
    EnvVarError(#[from] VarError),
}

/// Get an input value from the environment.
///
/// This function's behavior depends on the return type:
/// - `input::<String>("...")` will return `Err` if the input is missing.
/// - `input::<Option<String>>("...")` will return `Ok(None)` if the input is missing.
pub fn input<T: FromInputEnv>(name: &str) -> Result<T, Error> {
    T::from_input_env(name).map_err(Error::IoError)
}

/// Set an output value
pub fn output(name: &str, value: impl ToOutput) {
    println!("::set-output name={}::{}", name, value.to_output());
}
pub struct Repository {
    pub owner: String,
    pub name: String,
}

pub fn repository() -> Result<Repository, Error> {
    let repo_full = env::var("GITHUB_REPOSITORY")?;

    let (owner, name) = {
        let mut parts = repo_full.split('/');
        (parts.next().unwrap(), parts.next().unwrap())
    };

    Ok(Repository {
        owner: owner.to_owned(),
        name: name.to_owned(),
    })
}
