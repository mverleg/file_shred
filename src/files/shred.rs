use ::std::fs;
use ::std::fs::OpenOptions;
use ::std::io::{Seek, SeekFrom, Write};
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::rc::Rc;

use ::rand::RngCore;

use crate::util::base64::u64_to_base64str;
use crate::util::errors::add_err;
use crate::util::errors::wrap_io;
use crate::util::FedResult;

const SHRED_COUNT: u32 = 10;
const RENAME_COUNT: u32 = 10;

/// Shred a file, overwriting it with random data repeatedly, and subsequently deleting.
pub fn delete_file(path: &Path, verbose: bool) -> FedResult<()> {
    match OpenOptions::new().read(false).write(true).append(false).open(path) {
        Ok(mut file) => {
            let file_meta = wrap_io(|| "could not inspect file", file.metadata())?;
            assert!(file_meta.is_file());
            let file_size = file_meta.len();
            debug_assert!(SHRED_COUNT > 4);
            overwrite_constant(&mut file, file_size, verbose, 0)?;  // 00000000
            wrap_io(|| "could not persist file while shredding", file.sync_data())?;
            overwrite_constant(&mut file, file_size, verbose, 255)?;  // 11111111
            wrap_io(|| "could not persist file while shredding", file.sync_data())?;
            overwrite_constant(&mut file, file_size, verbose, 85)?;  // 01010101
            wrap_io(|| "could not persist file while shredding", file.sync_data())?;
            overwrite_constant(&mut file, file_size, verbose, 170)?;  // 10101010
            wrap_io(|| "could not persist file while shredding", file.sync_data())?;
            for _ in 0..SHRED_COUNT - 4 {
                overwrite_random_data(&mut file, file_size, verbose)?;
                wrap_io(|| "could not persist file while shredding", file.sync_data())?;
            }
        },
        Err(err) => return Err(add_err(format!("could not remove file '{}' because \
            it could not be opened in write mode", path.to_string_lossy()), verbose, err)),
    }
    //TODO @mark: remove meta data
    let renamed_path = repeatedly_rename_file(path, RENAME_COUNT, verbose)?;
    match fs::remove_file(&renamed_path) {
        Ok(_) => Ok(()),
        Err(err) => return Err(add_err(format!("could not remove file '{}' because \
            remove operation failed", &renamed_path.to_string_lossy()), verbose, err)),
    }
}

fn overwrite_constant<F: Write + Seek>(
    file: &mut F,
    file_size: u64,
    verbose: bool,
    value: u8,
) -> FedResult<()> {
    let data = Rc::new([value; 512]);
    overwrite_data(file, file_size, verbose, || data.clone())
}

fn overwrite_random_data<F: Write + Seek>(
    file: &mut F,
    file_size: u64,
    verbose: bool,
) -> FedResult<()> {
    let mut rng = rand::thread_rng();
    overwrite_data(file, file_size, verbose, || {
        let mut data = [0u8; 512];
        rng.fill_bytes(&mut data);
        Rc::new(data)
    })
}

/// Overwrite the data with garbage.
/// It is recommended to sync the file after each step.
fn overwrite_data<F: Write + Seek>(
    file: &mut F,
    file_size: u64,
    verbose: bool,
    mut value_gen: impl FnMut() -> Rc<[u8; 512]>,
) -> FedResult<()> {
    // Jump to start of file
    match file.seek(SeekFrom::Start(0)) {
        Ok(size) => assert_eq!(size, 0),
        Err(err) => return Err(add_err("could not just to start of file during shredding", verbose, err)),
    }

    // Overwrite the data in blocks. Might overwrite a bit at the end.
    let steps = (file_size + 511) / 512;
    dbg!(steps);
    for _ in 0..steps {
        match file.write(&*value_gen()) {
            Ok(size) => assert_eq!(size, 512),
            Err(err) => return Err(add_err("could not overwrite file during shredding", verbose, err)),
        }
    }

    Ok(())
}

fn repeatedly_rename_file(original_pth: &Path, reps: u32, verbose: bool,) -> FedResult<PathBuf> {
    let mut renamed = reps;
    let mut old_path = original_pth.to_owned();
    for iter in 0..100*reps {
        let name = format!("{}.tmp", &u64_to_base64str(iter as u64)[0..4]);
        let new_path = {
            let mut p = old_path.clone();
            p.set_file_name(name);
            p
        };
        if new_path.exists() {
            continue;
        }
        match fs::rename(&old_path, &new_path) {
            Ok(()) => {},
            Err(err) => return Err(add_err("failed to rename file during shredding", verbose, err)),
        }
        old_path =
            new_path;
        renamed -= 1;
        if renamed == 0 {
            break;
        }
    }
    Ok(old_path)
}

//TODO @mark: some shredders also do renames, should I do that?

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // #[test]
    // //TODO @mark: TEMPORARY! REMOVE THIS!
    // fn overwrite_fixed_tmp() {
    //     // A bunch of scopes here, in an attempt to close the file after each step.
    //     let in_pth = NamedTempFile::new().unwrap();
    //     {
    //         fs::write(&in_pth, b"hello world").unwrap();
    //     }
    //     {
    //         let mut file = OpenOptions::new().read(false).write(true).append(false).open(&in_pth).unwrap();
    //         let file_size = file.metadata().unwrap().len();
    //         dbg!(file_size);  //TODO @mark: TEMPORARY! REMOVE THIS!
    //         overwrite_constant(&mut file, file_size, true, 85).unwrap();
    //         dbg!(file.metadata().unwrap().len());  //TODO @mark: TEMPORARY! REMOVE THIS!
    //     }
    //     let data = {
    //         fs::read(&in_pth).unwrap()
    //     };
    //     assert!(!data.starts_with(b"hello world"));
    //     assert!(data.starts_with(b"hello"));
    // }

    #[test]
    fn overwrite_fixed() {
        // A bunch of scopes here, in an attempt to close the file after each step.
        let mut mock_file = Cursor::new(b"hello world".to_vec());
        overwrite_constant(&mut mock_file, 11, true, 85).unwrap();
        let data = mock_file.get_ref();
        assert!(!data.starts_with(b"hello world"));
        assert!(data.starts_with(b"UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU"));
        assert_eq!(data.len(), 512);
    }

    #[test]
    fn overwrite_random() {
        unimplemented!();
    }

    #[test]
    fn rename_collision() {
        unimplemented!();
    }
}
