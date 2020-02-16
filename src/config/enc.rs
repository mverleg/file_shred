use ::std::path::Path;
use ::std::path::PathBuf;

use crate::key::Key;

#[derive(Debug)]
pub struct EncryptConfig {
    files: Vec<PathBuf>,
    raw_key: Key,
    debug: bool,
    overwrite: bool,
    delete_input: bool,
    output_dir: Option<PathBuf>,
    output_extension: String,
    dry_run: bool,
}

impl EncryptConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        files: Vec<PathBuf>,
        raw_key: Key,
        debug: bool,
        mut overwrite: bool,
        mut delete_input: bool,
        output_dir: Option<PathBuf>,
        output_extension: String,
        dry_run: bool,
    ) -> Self {
        assert!(!files.is_empty());
        if dry_run {
            delete_input = false;
            overwrite = false;
        }
        EncryptConfig {
            files,
            raw_key,
            debug,
            overwrite,
            delete_input,
            output_dir,
            output_extension,
            dry_run,
        }
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn raw_key(&self) -> &Key {
        &self.raw_key
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn overwrite(&self) -> bool {
        self.overwrite
    }

    pub fn delete_input(&self) -> bool {
        self.delete_input
    }

    pub fn output_dir(&self) -> Option<&Path> {
        match &self.output_dir {
            Some(dir) => Some(dir.as_path()),
            None => None,
        }
    }

    pub fn output_extension(&self) -> &str {
        &self.output_extension
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}
