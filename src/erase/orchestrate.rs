use ::std::fs;
use ::std::fs::OpenOptions;
use ::std::path::Path;

use crate::config::conf::ShredConfig;
use crate::erase::metadata::remove_file_times;
use crate::erase::overwrite::{overwrite_constant, overwrite_random_data};
use crate::erase::rename::repeatedly_rename_file;
use crate::util::errors::add_err;
use crate::util::errors::wrap_io;
use crate::util::FedResult;

//TODO @mark: configurable
const SHRED_COUNT: u32 = 10;
const RENAME_COUNT: u32 = 10;

//TODO @mark: option to not delete file

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path, config: &ShredConfig) -> FedResult<()> {
    let verbose = config.verbosity.debug();
    match OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .open(path)
    {
        Ok(mut file) => {
            let file_meta = wrap_io(|| "could not inspect file", file.metadata())?;
            assert!(file_meta.is_file());
            let file_size = file_meta.len();
            debug_assert!(SHRED_COUNT > 4);
            overwrite_constant(&mut file, file_size, verbose, 0)?; // 00000000
            wrap_io(
                || "could not persist file while shredding",
                file.sync_data(),
            )?;
            overwrite_constant(&mut file, file_size, verbose, 255)?; // 11111111
            wrap_io(
                || "could not persist file while shredding",
                file.sync_data(),
            )?;
            overwrite_constant(&mut file, file_size, verbose, 85)?; // 01010101
            wrap_io(
                || "could not persist file while shredding",
                file.sync_data(),
            )?;
            overwrite_constant(&mut file, file_size, verbose, 170)?; // 10101010
            wrap_io(
                || "could not persist file while shredding",
                file.sync_data(),
            )?;
            for _ in 0..SHRED_COUNT - 4 {
                overwrite_random_data(&mut file, file_size, verbose)?;
                wrap_io(
                    || "could not persist file while shredding",
                    file.sync_data(),
                )?;
            }
        }
        Err(err) => {
            return Err(add_err(
                format!(
                    "could not remove file '{}' because \
            it could not be opened in write mode",
                    path.to_string_lossy()
                ),
                verbose,
                err,
            ))
        }
    }
    //TODO @mark: remove meta data
    //TODO @mark: https://docs.rs/filetime/0.2.8/filetime/
    remove_file_times(&path, verbose)?;
    let renamed_path = repeatedly_rename_file(path, RENAME_COUNT, verbose)?;
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
