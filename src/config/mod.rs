use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::EndecConfig;
use crate::header::strategy::Verbosity;
use crate::key::Key;

#[derive(Debug)]
pub struct ShredConfig {
    pub files: Vec<PathBuf>,
    pub verbosity: Verbosity,
    pub keep_files: bool,
}

impl ShredConfig {
    pub fn new(
        files: Vec<PathBuf>,
        verbosity: Verbosity,
        keep_files: bool,
    ) -> Self {
        ShredConfig {
            files,
            verbosity,
            keep_files,
        }
    }
}

impl EndecConfig for ShredConfig {
    fn files(&self) -> &[PathBuf] {
        &self.files
    }

    fn raw_key(&self) -> &Key {
        &self.raw_key
    }

    fn verbosity(&self) -> Verbosity {
        self.verbosity
    }

    fn overwrite(&self) -> bool {
        self.overwrite
    }

    fn delete_input(&self) -> bool {
        self.delete_input
    }

    fn output_dir(&self) -> Option<&Path> {
        match &self.output_dir {
            Some(dir) => Some(dir.as_path()),
            None => None,
        }
    }
}
