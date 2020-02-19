use ::std::io;

pub type FedResult<T> = Result<T, String>;

pub fn wrap_io<T>(base_msg: &str, res: io::Result<T>) -> FedResult<T> {
    match res {
        Ok(val) => FedResult::Ok(val),
        Err(val) => FedResult::Err(format!("{}: {}", base_msg, val)),
    }
}
