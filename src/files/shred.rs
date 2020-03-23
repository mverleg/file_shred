use crate::util::FedResult;
use std::path::Path;
use std::fs;
use std::fs::{OpenOptions, File};
use crate::util::errors::wrap_io;
use std::os::ios::fs::MetadataExt;
use std::io::Write;

const SHRED_COUNT: u32 = 20;

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path) -> FedResult<()> {
    match OpenOptions::new().read(false).write(true).append(false).open(path) {
        Ok(mut file) => {
            rng = unimplemented!();
            let file_meta = wrap_io("could not inspect file", file.metadata())?;
            assert!(file_meta.is_file());
            let file_size = file_meta.len();
            overwrite_constant(&mut file, file_size, 0)?;  // 00000000
            overwrite_constant(&mut file, file_size, 255)?;  // 11111111
            overwrite_constant(&mut file, file_size, 85)?;  // 01010101
            overwrite_constant(&mut file, file_size, 170)?;  // 10101010
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

fn overwrite_constant(
    file: &mut File,
    file_size: u64,
    value: u8,
) -> FedResult<()> {
    // let data = [0u8; 32];
    // file.write_all(&data);
    
}

fn overwrite_random_data(
    file: &mut File,
    file_size: u64,
) -> FedResult<()> {


}

//TODO @mark: tests
fn overwrite_data(
    file: &mut File,
    value: Vec<u8>,
) -> FedResult<()> {
    //TODO @mark: jump to start
    let mut remaining = file_size;
    while remaining >= ::std::u32::MAX {
        let all_data = vec![value; ::std::u32::MAX];
        for _ in 0..file_size {
            match file.write(&all_data) {
                Ok(size) => assert!(size == ::std::u32::MAX),
                Err(_) => return Err(format!("could not shred file{}",
                    if verbose {  &format!("; reason: {:?}", err) } else { "" })),
            }
        }
        remaining -= ::std::u32::MAX;
    }
    //TODO @mark: flush
    unimplemented!()
}