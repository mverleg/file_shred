use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::number2name::BASE32HUMAN;

use crate::util::errors::add_err;
use crate::util::ShredResult;

fn generate_name(number: u32) -> String {
    // Character set: abcdefghjkmnpqrstuvwxyz23456789_
    let mut name = BASE32HUMAN.encode(number as u64);
    name.push_str(".tmp");
    name
}

pub fn repeatedly_rename_file(original_pth: &Path, reps: u32, verbose: bool) -> ShredResult<PathBuf> {
    let mut renamed = reps;
    let mut old_path = original_pth.to_owned();
    for iter in 0..100 * reps {
        let new_path = {
            let mut p = old_path.clone();
            p.set_file_name(generate_name(iter));
            p
        };
        if new_path.exists() {
            continue;
        }
        match fs::rename(&old_path, &new_path) {
            Ok(()) => {}
            Err(err) => {
                return Err(add_err(
                    "failed to rename file during shredding",
                    verbose,
                    err,
                ))
            }
        }
        old_path = new_path;
        renamed -= 1;
        if renamed == 0 {
            break;
        }
    }
    Ok(old_path)
}

#[cfg(test)]
mod tests {

    use ::tempfile::tempdir;

    use super::*;

    #[test]
    fn rename() {
        let data = b"hello world, this is test data";
        let temp_handle = tempdir().unwrap();
        let mut path = temp_handle.path().to_owned();
        path.push("original.file");
        fs::write(&path, &data).unwrap();
        let new_pth = repeatedly_rename_file(&path, 5, true).unwrap();
        assert_eq!("e.tmp", new_pth.file_name().unwrap());
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
        make_collision_file(&path, "e.tmp");
        make_collision_file(&path, "z.tmp");
        make_collision_file(&path, "_.tmp");
        make_collision_file(&path, "aa.tmp");
        path.push("original.file");
        fs::write(&path, &data).unwrap();
        let new_pth = repeatedly_rename_file(&path, 30, true).unwrap();
        assert_eq!("ab.tmp", new_pth.file_name().unwrap());
        assert_eq!(&*data, fs::read(new_pth).unwrap().as_slice());
    }
}
