use ::std::env;
use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;
use std::io::{BufRead, stdin};
use std::str::FromStr;

use ::secstr::SecStr;

use crate::util::FedResult;

#[derive(Debug)]
pub enum KeySource {
    CliArg(SecStr),
    EnvVar(String),
    File(PathBuf),
    AskTwice,
    AskOnce,
    Pipe,
}

impl FromStr for KeySource {
    type Err = String;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        if txt.starts_with("arg:") {
            return Ok(KeySource::CliArg(SecStr::from(txt[4..].to_owned())));
        }
        if txt.starts_with("env:") {
            return Ok(KeySource::EnvVar(txt[4..].to_owned()));
        }
        if txt.starts_with("file:") {
            return Ok(KeySource::File(PathBuf::from(txt[5..].to_owned())));
        }
        if txt == "ask" {
            return Ok(KeySource::AskTwice);
        }
        if txt == "ask-once" {
            return Ok(KeySource::AskOnce);
        }
        if txt == "pipe" {
            return Ok(KeySource::Pipe);
        }
        Err(format!("key string was not recognized; got '{}', should be one of \
        'arg:$password', 'env:$var_name', 'file:$path', 'ask', 'askonce', 'pipe'", txt))
    }
}

fn key_from_env_var(env_var_name: &str) -> FedResult<SecStr> {
    match env::var(env_var_name) {
        Ok(env_var_value) => Ok(SecStr::from(env_var_value.trim())),
        Err(err) => match err {
            env::VarError::NotPresent => Err(format!(
                "could not find environment variable named '{}' (which is \
                            expected to contain the encryption key)", env_var_name)),
            env::VarError::NotUnicode(_) => Err(format!(
                "environment variable named '{}' did not contain valid data (it \
                            is expected to contain the encryption key, which must be unicode)",
                env_var_name)),
        }
    }
}

fn key_from_file(file_path: &Path) -> FedResult<SecStr> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(SecStr::from(content.trim())),
        Err(io_err) => Err(format!("failed to read encryption key from \
                        file '{}'; reason: {}", file_path.to_string_lossy(), io_err)),
    }
}

fn ask_key_from_prompt(message: &str) -> FedResult<SecStr> {
    match rpassword::read_password_from_tty(Some(message)) {
        Ok(pw) => Ok(SecStr::from(pw.trim())),
        Err(_) => Err(format!("failed to get password from interactive console")),
    }
}

fn key_from_prompt(ask_twice: bool) -> FedResult<SecStr> {
    if ask_twice {
        let pw1 = ask_key_from_prompt("key: ")?;
        let pw2 = ask_key_from_prompt("repeat key: ")?;
        if pw1 != pw2 {
            return Err("passwords did not match".to_owned());
        }
        Ok(pw2)
    } else {
        ask_key_from_prompt("key: ")
    }
}

fn key_from_pipe() -> FedResult<SecStr> {
    let mut pw = String::new();
    match stdin().lock().read_line(&mut pw) {
        Ok(count) => match count >= 1 {
            true => Ok(SecStr::from(pw.trim())),
            false => Err("no key was piped into the program".to_owned()),
        },
        Err(_) => Err("failed to read data piped into the program".to_owned()),
    }
}

impl KeySource {
    /// Obtain the key, which might involve IO.
    pub fn obtain_key(self) -> FedResult<SecStr> {
        match self {
            KeySource::CliArg(pw) => Ok(pw),
            KeySource::EnvVar(env_var_name) => key_from_env_var(&env_var_name),
            KeySource::File(file_path) => key_from_file(&file_path),
            KeySource::AskOnce => key_from_prompt(false),
            KeySource::AskTwice => key_from_prompt(true),
            KeySource::Pipe => key_from_pipe(),
        }
    }
}