use ::std::fmt;
use ::std::io;

pub type ShredResult<T> = Result<T, String>;

/// Change IO error into FedResult error.
pub fn wrap_io<T, S: AsRef<str>>(
    base_msg: impl FnOnce() -> S,
    res: io::Result<T>,
) -> ShredResult<T> {
    match res {
        Ok(val) => ShredResult::Ok(val),
        Err(val) => ShredResult::Err(format!("{}: {}", base_msg().as_ref(), val)),
    }
}

/// Push error message at end of string if verbose logging is enabled.
pub fn add_err(msg: impl Into<String>, verbose: bool, err: impl fmt::Debug) -> String {
    let mut msg = msg.into();
    if verbose {
        msg.push_str("; details: ");
        msg.push_str(&format!("{:?}", err));
    }
    msg
}
