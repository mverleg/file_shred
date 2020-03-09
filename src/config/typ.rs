use ::std::fmt::Debug;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::header::strategy::Verbosity;
use crate::key::Key;

#[derive(Debug, Clone, Copy)]
pub enum Extension<'a> {
    Add(&'a str),
    Strip,
}

pub trait EndecConfig: Debug {
    fn files(&self) -> &[PathBuf];

    fn raw_key(&self) -> &Key;

    fn verbosity(&self) -> Verbosity;

    fn debug(&self) -> bool {
        self.verbosity().debug()
    }

    fn quiet(&self) -> bool {
        self.verbosity().quiet()
    }

    fn overwrite(&self) -> bool;

    fn delete_input(&self) -> bool;

    fn output_dir(&self) -> Option<&Path>;
}

#[cfg(test)]
#[derive(Debug)]
pub struct MockEndecConfig {
    pub files: Vec<PathBuf>,
    pub raw_key: Key,
    pub verbosity: Verbosity,
    pub overwrite: bool,
    pub delete_input: bool,
    pub output_dir: Option<PathBuf>,
}

#[cfg(test)]
impl EndecConfig for MockEndecConfig {
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
