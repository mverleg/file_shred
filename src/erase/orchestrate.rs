use ::std::path::Path;

use crate::config::conf::ShredConfig;
use crate::erase::metadata::remove_file_times;
use crate::erase::overwrite::repeatedly_overwrite;
use crate::erase::remove::{remove_file, truncate_file};
use crate::erase::rename::repeatedly_rename_file;
use crate::util::ShredResult;

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path, config: &ShredConfig) -> ShredResult<()> {
    let verbose = config.verbosity.debug();

    // Overwrite the file.
    repeatedly_overwrite(path, config.overwrite_count, verbose)?;
    if config.keep_files {
        return Ok(());
    }

    // Remove metadata.
    //TODO @mark: remove permissions (on some platforms?)
    remove_file_times(&path, verbose)?;

    // Rename the file.
    let renamed_path = repeatedly_rename_file(path, config.rename_count, verbose)?;

    // Delete the file
    truncate_file(&renamed_path, verbose)?;
    remove_file(&renamed_path, verbose)
}
