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
use filetime::{set_file_times, FileTime};

const SHRED_COUNT: u32 = 10;
const RENAME_COUNT: u32 = 10;

//TODO @mark: make this into separate lib?
//TODO @mark: add CLI option for shredding?

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
    //TODO @mark: https://docs.rs/filetime/0.2.8/filetime/
    remove_file_times(&path, verbose)?;
    let renamed_path = repeatedly_rename_file(path, RENAME_COUNT, verbose)?;
    //TODO @mark: truncate the file
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
    for _ in 0..steps {
        match file.write(&*value_gen()) {
            Ok(size) => assert_eq!(size, 512),
            Err(err) => return Err(add_err("could not overwrite file during shredding", verbose, err)),
        }
    }

    Ok(())
}

fn remove_file_times(path: &Path, verbose: bool) -> FedResult<()> {
    match set_file_times(path, FileTime::zero(), FileTime::zero()) {
        Ok(()) => Ok(()),
        Err(err) => return Err(add_err("failed to set file permissions while shredding", verbose, err)),
    }
}

fn repeatedly_rename_file(original_pth: &Path, reps: u32, verbose: bool) -> FedResult<PathBuf> {
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

#[cfg(test)]
mod tests {
    use ::std::io::Cursor;

    use ::tempfile::tempdir;

    use super::*;

    #[test]
    fn overwrite_long() {
        let mut mock_file = Cursor::new(vec![0u8; 65_536 + 1]);
        overwrite_constant(&mut mock_file, 65_536 + 1, false, 'm' as u8).unwrap();
        let data = mock_file.get_ref();
        assert!(data.starts_with(b"mmmmmm"));
        assert!(data.ends_with(b"mmmmmm"));
        assert_eq!(data.len(), 65_536 + 512);
    }

    #[test]
    fn overwrite_fixed() {
        let mut mock_file = Cursor::new(b"hello world".to_vec());
        overwrite_constant(&mut mock_file, 11, true, 85).unwrap();
        let data = mock_file.get_ref();
        assert!(!data.starts_with(b"hello world"));
        assert!(data.starts_with(b"UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU"));
        assert_eq!(data.len(), 512);
    }

    #[test]
    fn overwrite_random() {
        let initial = b"hello world this is an unlikely message that shouldn't happen by chance!";
        let mut mock_file = Cursor::new(initial.to_vec());
        overwrite_constant(&mut mock_file, 11, true, 85).unwrap();
        let data = mock_file.get_ref();
        assert!(!data.starts_with(initial));
        assert_eq!(data.len(), 512);
    }

    #[test]
    fn rename() {
        let data = b"hello world, this is test data";
        let temp_handle = tempdir().unwrap();
        let mut path = temp_handle.path().to_owned();
        path.push("original.file");
        fs::write(&path, &data).unwrap();
        let new_pth = repeatedly_rename_file(&path, 5, true).unwrap();
        assert_eq!("BAAA.tmp", new_pth.file_name().unwrap());
        assert_eq!(&*data, fs::read(new_pth).unwrap().as_slice());
    }

    #[test]
    fn rename_collision() {
        fn make_collision_file(dir: &Path, name: &str) {
            let mut path = dir.to_owned();
            path.push(name);
            let data = format!("collision data at {}", path.to_string_lossy());
            fs::write(&path, &data).unwrap();
        }
        let data = b"hello world, this is test data";
        let temp_handle = tempdir().unwrap();
        let mut path = temp_handle.path().to_owned();
        make_collision_file(&path, "AQAA.tmp");
        make_collision_file(&path, "BAAA.tmp");
        path.push("original.file");
        fs::write(&path, &data).unwrap();
        let new_pth = repeatedly_rename_file(&path, 5, true).unwrap();
        assert_eq!("BgAA.tmp", new_pth.file_name().unwrap());
        assert_eq!(&*data, fs::read(new_pth).unwrap().as_slice());
    }
}
