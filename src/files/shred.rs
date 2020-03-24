use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use crate::util::errors::wrap_io;
use crate::util::FedResult;
use rand::RngCore;

const SHRED_COUNT: u32 = 10;

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path, verbose: bool) -> FedResult<()> {
    match OpenOptions::new().read(false).write(true).append(false).open(path) {
        Ok(mut file) => {
            let file_meta = wrap_io(|| "could not inspect file", file.metadata())?;
            assert!(file_meta.is_file());
            let file_size = file_meta.len();
            debug_assert!(SHRED_COUNT > 4);
            overwrite_constant(&mut file, file_size, verbose, 0)?;  // 00000000
            overwrite_constant(&mut file, file_size, verbose, 255)?;  // 11111111
            overwrite_constant(&mut file, file_size, verbose, 85)?;  // 01010101
            overwrite_constant(&mut file, file_size, verbose, 170)?;  // 10101010
            for _ in 0..SHRED_COUNT - 4 {
                overwrite_random_data(&mut file, file_size, verbose)?;
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

fn overwrite_constant(
    file: &mut File,
    file_size: u64,
    verbose: bool,
    value: u8,
) -> FedResult<()> {
    let data = [value; 512];
    overwrite_data(file, file_size, verbose, &|| &data)
}

fn overwrite_random_data(
    file: &mut File,
    file_size: u64,
    verbose: bool,
) -> FedResult<()> {
    let mut data = [0u8; 512];
    let mut rng = rand::thread_rng();
    overwrite_data(file, file_size, verbose, &|| {
        rng.fill_bytes(&mut data);
        &data
    })
}

//TODO @mark: tests
fn overwrite_data<'a>(
    file: &mut File,
    file_size: u64,
    verbose: bool,
    value_gen: &'a impl FnMut() -> &'a [u8; 512],
) -> FedResult<()> {
    // Jump to start of file
    match file.seek(SeekFrom::Start(0)) {
        Ok(size) => assert!(size == 0),
        Err(err) => return Err(format!("could not just to start of file during shredding{}",
            if verbose {  &format!("; reason: {:?}", err) } else { "" })),
    }

    // Overwrite the data in blocks. Might overwrite a bit at the end.
    let steps = (file_size + 511) / 512;
    for _ in 0..steps {
        for _ in 0..file_size {
            match file.write(value_gen()) {
                Ok(size) => assert_eq!(size, 512),
                Err(err) => return Err(format!("could not overwrite file during shredding{}",
                    if verbose {  &format!("; reason: {:?}", err) } else { "" })),
            }
        }
    }

    // Flush to make sure changes are written (barring OS cache)
    match file.sync_data() {
        Ok(size) => Ok(()),
        Err(err) => Err(format!("could not just to start of file during shredding{}",
            if verbose {  &format!("; reason: {:?}", err) } else { "" })),
    }
}

//TODO @mark: some shredders also do renames, should I do that?
