use crate::util::FedResult;
use std::path::Path;
use std::fs;
use std::fs::{OpenOptions, File};
use crate::util::errors::wrap_io;
use std::os::ios::fs::MetadataExt;

const SHRED_COUNT: u32 = 20;

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path) -> FedResult<()> {
    match OpenOptions::new().read(false).write(true).append(false).open(path) {
        Ok(file) => {
            rng = unimplemented!();
            let file_meta = wrap_io("could not inspect file", file.metadata())?;
            assert!(file_meta.is_file());
            let file_size = file_meta.len();
            for _ in 0..SHRED_COUNT {
                overwrite_random_data(&mut file, file_size)?;
            }
        },
        Err(err) => return Err(format!("could not remove file '{}' because \
            it could not be opened in write mode{}", path.to_string_lossy(),
            if verbose {  &format!("; reason: {:?}", err) } else { "" })),
    }
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(format!("could not remove file '{}' because \
            remove operation failed{}", path.to_string_lossy(),
            if verbose {  &format!("; reason: {:?}", err) } else { "" })),
    }
}

fn overwrite_random_data(
    file: &mut File,
    file_size: usize,
) -> FedResult<()> {




    //TODO @mark: flush
}