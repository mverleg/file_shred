use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::{EndecConfig, Extension};
use crate::header::strategy::Verbosity;
use crate::key::Key;

#[derive(Debug)]
pub struct DecryptConfig {
    files: Vec<PathBuf>,
    raw_key: Key,
    verbosity: Verbosity,
    overwrite: bool,
    delete_input: bool,
    output_dir: Option<PathBuf>,
}

impl DecryptConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        files: Vec<PathBuf>,
        raw_key: Key,
        verbosity: Verbosity,
        overwrite: bool,
        delete_input: bool,
        output_dir: Option<PathBuf>,
    ) -> Self {
        assert!(!files.is_empty());
        DecryptConfig {
            files,
            raw_key,
            verbosity,
            overwrite,
            delete_input,
            output_dir,
        }
    }

    pub fn output_dir(&self) -> Option<&Path> {
        match &self.output_dir {
            Some(dir) => Some(dir.as_path()),
            None => None,
        }
    }
}

impl EndecConfig for DecryptConfig {
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
            Some(pth) => Some(pth),
            None => None
        }
    }
}
