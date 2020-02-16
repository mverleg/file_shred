use ::std::fmt::Debug;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::key::Key;

pub trait EndecConfig: Debug {
    fn files(&self) -> &[PathBuf];

    fn raw_key(&self) -> &Key;

    fn debug(&self) -> bool;

    fn overwrite(&self) -> bool;

    fn delete_input(&self) -> bool;

    fn output_dir(&self) -> Option<&Path>;

    fn extension(&self) -> &str;
}
