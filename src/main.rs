use ::std::fmt;
use ::std::io::stderr;
use ::std::io::Write;
use ::std::path::PathBuf;
use ::std::process::exit;

use ::file_endec::config::EncryptConfig;
use ::file_endec::encrypt;
use ::file_endec::header::strategy::Verbosity;
use ::file_endec::key::Key;
use ::file_endec::key::KeySource;
use ::file_endec::util::FedResult;
use ::structopt::StructOpt;
use file_shred::util::FedResult;

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
    //TODO @mark: use
}

impl fmt::Display for ShredArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("  files:\n")?;
        for file in self.files.clone().into_iter() {
            f.write_str("  - ")?;
            f.write_str(file.to_string_lossy().as_ref())?;
            f.write_str("\n")?;
        }

        // Currently, this is always "on", because printing is only used in debug mode.
        f.write_str("  logging: ")?;
        f.write_str(if self.debug { "verbose (debug)" } else { if self.quiet { "quiet" } else { "standard" } })?;
        f.write_str("\n")?;

        f.write_str("  after overwrite: ")?;
        f.write_str(if self.keep { "keep" } else { "delete" })?;
        f.write_str("\n")?;

        Ok(())
    }
}

pub fn main() {
    if let Err(err) = go_encrypt() {
        stderr().write_all(err.as_bytes()).unwrap();
        exit(1);
    }
}

impl ShredArguments {
    fn convert(self) -> FedResult<EncryptConfig> {
        let verbosity = match (self.debug, self.quiet) {
            (true, true) => return Err("cannot use quiet mode and debug mode together".to_owned()),
            (true, false) => Verbosity::Debug,
            (false, true) => Verbosity::Quiet,
            (false, false) => Verbosity::Normal,
        };
        let extension = if self.output_extension.starts_with('.') {
            self.output_extension
        } else {
            format!(".{}", self.output_extension)
        };
        Ok(ShredConfig::new(
            self.files,
            verbosity,
            self.keep,
        ))
    }
}

fn go_encrypt() -> FedResult<()> {
    let args = ShredArguments::from_args();
    if args.debug {
        println!("arguments provided:\n{}", args);
    }
    let config = args.convert(key)?;
    shred(&config)
}

#[cfg(test)]
mod tests {
    use ::file_endec::config::typ::EndecConfig;
    use ::file_endec::header::strategy::Verbosity;
    use ::file_endec::key::Key;

    use super::*;

    #[test]
    fn parse_args_minimal() {
        let args = ShredArguments::from_iter(&["shred", "file.txt"]);
        let config = args.convert().unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert_eq!(config.verbosity(), Verbosity::Normal);
        assert!(!config.keep());
    }

    #[test]
    fn parse_args_long() {
        let args = ShredArguments::from_iter(&[
            "shred",
            "file.txt",
            "-q",
            "-k",
            "/tmp/hello",
            "another_file.txt",
            "there_are_three_files",
        ]);
        let config = args.convert().unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert!(config.files().contains(&PathBuf::from("another_file.txt")));
        assert!(config.files().contains(&PathBuf::from("there_are_three_files")));
        assert_eq!(3, config.files().len());
        assert_eq!(config.verbosity(), Verbosity::Quiet);
        assert!(config.keep());
    }
}