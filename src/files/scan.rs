#[cfg(test)] use ::std::env;
use ::std::ffi::OsStr;
use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;

#[cfg(test)] use ::lazy_static::lazy_static;

use crate::util::FedResult;

/// Recursively get all the files in a directory that have '.enc' extension.
pub fn get_enc_files_direct(dir: &Path) -> FedResult<Vec<PathBuf>> {
    let mut matches = vec![];
    match fs::read_dir(dir) {
        Ok(content) => {
            for path in content {
                match path {
                    Ok(path) => {
                        let path = path.path();
                        if !path.is_file() {
                            continue;
                        }
                        if let Some("enc") = path.extension().and_then(OsStr::to_str) {
                            matches.push(path.to_owned());
                        }
                    },
                    Err(err) => return Err(format!(
                        "Failed on entry in directory '{}' because '{}'",
                        dir.to_string_lossy(), err)),
                }
            }
        },
        Err(err) => return Err(format!(
            "Failed to read directory '{}' because '{}'",
            dir.to_string_lossy(), err)),
    }
    Ok(matches)
}

#[cfg(test)]
lazy_static! {
    static ref TEST_FILE_DIR: PathBuf = {
        // Try to find relative to target dir.
        let mut test_files_dir: PathBuf = {
            let mut p = PathBuf::from(std::env::current_exe().unwrap());
            p.pop();
            p.pop();
            p.pop();
            p.pop();
            p.push("test_files");
            p
        };
        let mut original_file = test_files_dir.clone();
        original_file.push("original.png");
        if !original_file.is_file() {
            // Perhaps the target dir is not in the default location, try something else.
            match env::var("ENDEC_TEST_FILE_DIR") {
                Ok(test_file_dir_env) => {
                    test_files_dir = PathBuf::from(test_file_dir_env);
                    original_file = test_files_dir.clone();
                    original_file.push("original.png");
                    if !original_file.is_file() {
                        panic!(format!("Expected test files at '{}' based on environment variable 'ENDEC_TEST_FILE_DIR', but the files were not found.", test_files_dir.to_string_lossy()));
                    }
                },
                Err(err) => panic!(format!("Expected test files at '{}' but they were not found; set the environment variable 'ENDEC_TEST_FILE_DIR' to the test file location.", test_files_dir.to_string_lossy())),
            }
        }
        test_files_dir
    };
}

#[cfg(test)]
pub fn test_files() -> Vec<PathBuf> {
    let test_files_dir = TEST_FILE_DIR.clone();
    get_enc_files_direct(&test_files_dir).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_files() {
        let files = get_enc_files_direct(&*TEST_FILE_DIR).unwrap();
        assert!(files.len() >= 1);
    }
}
