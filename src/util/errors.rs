use ::std::io;

pub type FedResult<T> = Result<T, String>;

pub fn wrap_io<T, S: AsRef<str>>(base_msg: impl FnOnce() -> S, res: io::Result<T>) -> FedResult<T> {
    match res {
        Ok(val) => FedResult::Ok(val),
        Err(val) => FedResult::Err(format!("{}: {}", base_msg().as_ref(), val)),
    }
}
