use ::std::fmt;
use ::std::io::stderr;
use ::std::io::Write;
use ::std::path::PathBuf;
use ::std::process::exit;

use ::structopt::StructOpt;

use ::file_shred::{shred, ShredConfig, ShredResult, Verbosity};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Shred",
    author = "github.com/mverleg/file_shred",
    about = "Securely erase one or more files."
)]
pub struct ShredArguments {
    #[structopt(
        name = "FILES",
        parse(from_os_str),
        required = true,
        min_values = 1,
        help = "One or more paths to input files (absolute or relative)"
    )]
    files: Vec<PathBuf>,

    #[structopt(
        short = "y",
        long = "no-confirm",
        help = "Delete files without asking for confirmation."
    )]
    no_confirm: bool,

    #[structopt(
        short = "v",
        long,
        help = "Show debug information, especially on errors."
    )]
    debug: bool,

    #[structopt(
        conflicts_with = "debug",
        short = "q",
        long = "quiet",
        help = "Do not show progress or other non-critical output."
    )]
    quiet: bool,

    #[structopt(
        short = "k",
        long = "keep",
        help = "Destroy the data, but do not rename or delete the file. Useful for non-regular files like special system devices."
    )]
    keep: bool,

    #[structopt(
        long = "overwrite-count",
        default_value = "10",
        help = "Number of times the file is overwritten (at least 1)."
    )]
    overwrite_count: u32,

    #[structopt(
        conflicts_with = "keep",
        long = "rename-count",
        help = "Number of times the file is renamed."
    )]
    rename_count: Option<u32>,
}

impl fmt::Display for ShredArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("  files:\n")?;
        for file in &self.files {
            f.write_str("  - ")?;
            f.write_str(file.to_string_lossy().as_ref())?;
            f.write_str("\n")?;
        }

        write!(
            f,
            "mode: {}",
            if self.no_confirm {
                "immediately delete"
            } else {
                "ask before deleting"
            }
        )?;

        // Currently, this is always "on", because printing is only used in debug mode.
        f.write_str("  logging: ")?;
        f.write_str(if self.debug {
            "verbose (debug)"
        } else if self.quiet {
            "quiet"
        } else {
            "standard"
        })?;
        f.write_str("\n")?;

        f.write_str("  after overwrite: ")?;
        f.write_str(if self.keep { "keep" } else { "delete" })?;
        f.write_str("\n")?;

        writeln!(f, "overwrite: {} times", self.overwrite_count)?;
        match self.rename_count {
            Some(rename_count) => writeln!(f, "rename: {} times\n", rename_count)?,
            None => write!(f, "rename: not applicable")?,
        };

        Ok(())
    }
}

pub fn main() {
    if let Err(err) = go_shred() {
        writeln!(stderr(), "{}", err).unwrap();
        exit(1);
    }
}

impl ShredArguments {
    fn convert(self) -> ShredResult<ShredConfig<PathBuf>> {
        let verbosity = match (self.debug, self.quiet) {
            (true, true) => return Err("cannot use quiet mode and debug mode together".to_owned()),
            (true, false) => Verbosity::Debug,
            (false, true) => Verbosity::Quiet,
            (false, false) => Verbosity::Normal,
        };
        if self.overwrite_count == 0 {
            return Err("overwrite-count is 0, but must be at least 1".to_owned());
        }
        let confirmation_prompt = !self.no_confirm;
        Ok(ShredConfig::interactive(
            self.files,
            confirmation_prompt,
            verbosity,
            self.keep,
            self.overwrite_count,
            self.rename_count.unwrap_or(10),
        ))
    }
}

fn go_shred() -> ShredResult<()> {
    let args = ShredArguments::from_args();
    if args.debug {
        println!("arguments provided:\n{}", args);
    }
    let config = args.convert()?;
    shred(&config)
}

#[cfg(test)]
mod tests {
    use crate::Verbosity;

    use super::*;

    #[test]
    fn parse_args_minimal() {
        let args = ShredArguments::from_iter(&["shred", "file.txt"]);
        let config = args.convert().unwrap();
        assert!(config.files.contains(&PathBuf::from("file.txt")));
        assert_eq!(1, config.files.len());
        assert_eq!(config.verbosity, Verbosity::Normal);
        assert!(!config.keep_files);
        assert_eq!(config.overwrite_count, 10);
        assert_eq!(config.rename_count, 10);
    }

    #[test]
    fn parse_args_long() {
        let args = ShredArguments::from_iter(&[
            "shred",
            "file.txt",
            "-q",
            "-k",
            "another_file.txt",
            "there_are_three_files",
            "--overwrite-count",
            "7",
        ]);
        let config = args.convert().unwrap();
        //TODO @mark: why so many &
        assert!(config.files.contains(&PathBuf::from("file.txt")));
        assert!(config.files.contains(&PathBuf::from("another_file.txt")));
        assert!(config
            .files
            .contains(&PathBuf::from("there_are_three_files")));
        assert_eq!(3, config.files.len());
        assert_eq!(config.verbosity, Verbosity::Quiet);
        assert!(config.keep_files);
        assert_eq!(config.overwrite_count, 7);
        assert_eq!(config.rename_count, 10);
    }
}
