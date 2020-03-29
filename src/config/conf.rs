use ::std::path::PathBuf;

use crate::config::typ::Verbosity;
#[derive(Debug)]
pub struct ShredConfig {
    pub files: Vec<PathBuf>,
    pub verbosity: Verbosity,
    pub keep_files: bool,
    pub overwrite_count: u32,
    pub rename_count: u32,
}

impl ShredConfig {
    pub fn new(
        files: Vec<PathBuf>,
        verbosity: Verbosity,
        keep_files: bool,
        overwrite_count: u32,
        rename_count: u32,
    ) -> Self {
        ShredConfig {
            files,
            verbosity,
            keep_files,
            overwrite_count,
            rename_count,
        }
    }
}
