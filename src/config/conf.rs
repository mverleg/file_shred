use ::std::path::PathBuf;

use crate::config::typ::Verbosity;
#[derive(Debug)]
pub struct ShredConfig {
    pub files: Vec<PathBuf>,
    pub verbosity: Verbosity,
    pub keep_files: bool,
}

impl ShredConfig {
    pub fn new(files: Vec<PathBuf>, verbosity: Verbosity, keep_files: bool) -> Self {
        ShredConfig {
            files,
            verbosity,
            keep_files,
        }
    }
}
