use ::std::path::PathBuf;

use crate::config::typ::Verbosity;
#[derive(Debug)]
pub struct ShredConfig {
    pub files: Vec<PathBuf>,
    pub confirmation_prompt: bool,
    pub verbosity: Verbosity,
    pub keep_files: bool,
    pub overwrite_count: u32,
    pub rename_count: u32,
    pub progress_bar: bool,
}

impl ShredConfig {
    pub fn non_interactive(
        files: Vec<PathBuf>,
        verbosity: Verbosity,
        keep_files: bool,
        overwrite_count: u32,
        rename_count: u32,
    ) -> Self {
        ShredConfig {
            files,
            confirmation_prompt: false,
            verbosity,
            keep_files,
            overwrite_count,
            rename_count,
            progress_bar: false,
        }
    }

    pub fn interactive(
        files: Vec<PathBuf>,
        confirmation_prompt: bool,
        verbosity: Verbosity,
        keep_files: bool,
        overwrite_count: u32,
        rename_count: u32,
    ) -> Self {
        ShredConfig {
            files,
            confirmation_prompt,
            verbosity,
            keep_files,
            overwrite_count,
            rename_count,
            progress_bar: !verbosity.quiet(),
        }
    }
}
