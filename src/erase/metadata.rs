use ::std::path::Path;

use ::filetime::{set_file_times, FileTime};

use crate::util::errors::add_err;
use crate::util::ShredResult;

/// Remove access and modification times by setting to zero timestamp.
pub fn remove_file_times(path: &Path, verbose: bool) -> ShredResult<()> {
    match set_file_times(path, FileTime::zero(), FileTime::zero()) {
        Ok(()) => Ok(()),
        Err(err) => {
            Err(add_err(
                "failed to set file permissions while shredding",
                verbose,
                err,
            ))
        }
    }
}
