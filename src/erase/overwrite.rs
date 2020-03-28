use ::std::io::{Seek, SeekFrom, Write};
use ::std::rc::Rc;

use ::rand::RngCore;

use crate::util::errors::add_err;
use crate::util::FedResult;

pub fn overwrite_constant<F: Write + Seek>(
    file: &mut F,
    file_size: u64,
    verbose: bool,
    value: u8,
) -> FedResult<()> {
    let data = Rc::new([value; 512]);
    overwrite_data(file, file_size, verbose, || data.clone())
}

pub fn overwrite_random_data<F: Write + Seek>(
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
        Err(err) => {
            return Err(add_err(
                "could not just to start of file during shredding",
                verbose,
                err,
            ))
        }
    }

    // Overwrite the data in blocks. Might overwrite a bit at the end.
    let steps = (file_size + 511) / 512;
    for _ in 0..steps {
        match file.write(&*value_gen()) {
            Ok(size) => assert_eq!(size, 512),
            Err(err) => {
                return Err(add_err(
                    "could not overwrite file during shredding",
                    verbose,
                    err,
                ))
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ::std::io::Cursor;

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
}
