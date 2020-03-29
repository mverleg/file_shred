use ::std::fs;
use ::std::fs::OpenOptions;
use ::std::path::Path;

use crate::config::conf::ShredConfig;
use crate::erase::metadata::remove_file_times;
use crate::erase::overwrite::{overwrite_constant, overwrite_random_data, repeatedly_overwrite};
use crate::erase::rename::repeatedly_rename_file;
use crate::util::errors::add_err;
use crate::util::ShredResult;

//TODO @mark: option to not delete file

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path, config: &ShredConfig) -> ShredResult<()> {
    let verbose = config.verbosity.debug();
    repeatedly_overwrite(path, config.overwrite_count, verbose)?;
    //TODO @mark: remove meta data
    //TODO @mark: https://docs.rs/filetime/0.2.8/filetime/
    remove_file_times(&path, verbose)?;
    let renamed_path = repeatedly_rename_file(path, config.rename_count, verbose)?;
    //TODO @mark: truncate the file
    match fs::remove_file(&renamed_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            Err(add_err(
                format!(
                    "could not remove file '{}' because \
                        remove operation failed",
                    &renamed_path.to_string_lossy()
                ),
                verbose,
                err,
            ))
        }
    }
}
