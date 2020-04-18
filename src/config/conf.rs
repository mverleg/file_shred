use ::std::path::Path;

use crate::config::typ::Verbosity;

#[derive(Debug)]
pub struct ShredConfig<'a, P: AsRef<Path> + ?Sized> {
    pub files: Vec<&'a P>,
    pub confirmation_prompt: bool,
    pub verbosity: Verbosity,
    pub keep_files: bool,
    pub overwrite_count: u32,
    pub rename_count: u32,
    pub progress_bar: bool,
}

impl <'a, P: AsRef<Path> + ?Sized> ShredConfig<'a, P> {
    pub fn non_interactive(
        files: Vec<&'a P>,
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
        files: Vec<&'a P>,
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
