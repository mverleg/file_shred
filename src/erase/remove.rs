use ::std::fs;
use ::std::fs::OpenOptions;
use ::std::path::Path;

use crate::util::errors::add_err;
use crate::ShredResult;

pub fn truncate_file(path: &Path, verbose: bool) -> ShredResult<()> {
    let file = match OpenOptions::new().write(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            return Err(add_err(
                "failed to open file for truncation".to_owned(),
                verbose,
                err,
            ))
        }
    };
    if let Err(err) = file.set_len(0) {
        return Err(add_err(
            "failed to truncation file".to_owned(),
            verbose,
            err,
        ));
    };
    Ok(())
}

pub fn remove_file(path: &Path, verbose: bool) -> ShredResult<()> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(add_err(
            format!(
                "could not remove file '{}' because remove operation failed",
                &path.to_string_lossy()
            ),
            verbose,
            err,
        )),
    }
}
