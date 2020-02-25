use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::EndecConfig;
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
    input_extension: String,
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
        input_extension: String,
    ) -> Self {
        assert!(!files.is_empty());
        DecryptConfig {
            files,
            raw_key,
            verbosity,
            overwrite,
            delete_input,
            output_dir,
            input_extension,
        }
    }

    pub fn output_dir(&self) -> Option<&Path> {
        match &self.output_dir {
            Some(dir) => Some(dir.as_path()),
            None => None,
        }
    }

    pub fn input_extension(&self) -> &str {
        &self.input_extension
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
        unimplemented!()
    }

    fn extension(&self) -> &str {
        self.input_extension()
    }
}
