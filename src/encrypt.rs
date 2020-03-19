use ::std::fmt;
use ::std::io::stderr;
use ::std::io::Write;
use ::std::path::PathBuf;
use ::std::process::exit;

use ::structopt::StructOpt;

use ::file_endec::config::EncryptConfig;
use ::file_endec::encrypt;
use ::file_endec::header::strategy::Verbosity;
use ::file_endec::key::Key;
use ::file_endec::key::KeySource;
use ::file_endec::util::FedResult;

//TODO @mark: flag like --read0 to accept pipe with \0 terminator byte like `find -print0` outputs (can't work with --key=pipe)

#[derive(Debug, StructOpt)]
#[structopt(
    name = "FileEnc",
    author = "github.com/mverleg/file_endec",
    about = "Securely encrypt one or more files using the given key."
)]
pub struct EncryptArguments {
    #[structopt(
        name = "FILES",
        parse(from_os_str),
        required = true,
        min_values = 1,
        help = "One or more paths to input files (absolute or relative)"
    )]
    files: Vec<PathBuf>,

    #[structopt(
        short = "k",
        long = "key",
        default_value = "ask",
        help = "Where to get the key; one of 'pass:$password', 'env:$var_name', 'file:$path', 'ask', 'askonce', 'pipe'"
    )]
    key_source: KeySource,

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

    #[structopt(short = "f", long, help = "Overwrite output files if they exist.")]
    overwrite: bool,

    #[structopt(
        short = "d",
        long,
        help = "Delete unencrypted input files after successful encryption (overwrites garbage before delete)."
    )]
    delete_input: bool,

    #[structopt(
        parse(from_os_str),
        short = "o",
        long,
        help = "Alternative output directory. If not given, output is saved alongside input."
    )]
    output_dir: Option<PathBuf>,

    #[structopt(
        long,
        default_value = ".enc",
        help = "Extension added to encrypted files."
    )]
    output_extension: String,

    #[structopt(
        long,
        help = "Test encryption, but do not save encrypted files (nor delete input, if --delete-input)."
    )]
    dry_run: bool,

    #[structopt(long, help = "Suppress warning if the encryption key is not strong.")]
    accept_weak_key: bool,
}

impl fmt::Display for EncryptArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("  files:\n")?;
        for file in self.files.clone().into_iter() {
            f.write_str("  - ")?;
            f.write_str(file.to_string_lossy().as_ref())?;
            f.write_str("\n")?;
        }

        //TODO @mark: absolute path?
        match &self.output_dir {
            Some(dir) => {
                f.write_str("  output directory: ")?;
                f.write_str(dir.to_string_lossy().as_ref())?
            }
            None => f.write_str("  output is stored alongside input")?,
        }
        f.write_str("\n")?;

        f.write_str("  extension: ")?;
        f.write_str(&self.output_extension)?;
        f.write_str("\n")?;

        // Currently, this is always "on", because printing is only used in debug mode.
        //TODO @mark: also include quiet mode (also for decrypt)
        f.write_str("  debug logging: ")?;
        f.write_str(if self.debug { "on" } else { "off" })?;
        f.write_str("\n")?;

        f.write_str("  dry run: ")?;
        f.write_str(if self.dry_run { "yes" } else { "no" })?;
        f.write_str("\n")?;

        f.write_str("  overwrite existing output files: ")?;
        f.write_str(if self.overwrite {
            if self.dry_run {
                "no (overridden by dry run)"
            } else {
                "yes"
            }
        } else {
            "no"
        })?;
        f.write_str("\n")?;

        f.write_str("  delete input files: ")?;
        f.write_str(if self.delete_input {
            if self.dry_run {
                "no (overridden by dry run)"
            } else {
                "yes"
            }
        } else {
            "no"
        })?;

        Ok(())
    }
}

pub fn main() {
    if let Err(err) = go_encrypt() {
        stderr().write_all(err.as_bytes()).unwrap();
        exit(1);
    }
}

impl EncryptArguments {
    fn convert(self, key: Key) -> FedResult<EncryptConfig> {
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
        Ok(EncryptConfig::new(
            self.files,
            key,
            verbosity,
            self.overwrite,
            self.delete_input,
            self.output_dir,
            extension,
            self.dry_run,
        ))
    }
}

//TODO: if wildcards or directories are ever supported, then skip files that have the encrypted extension (i.e. .enc)

fn go_encrypt() -> FedResult<()> {
    let args = EncryptArguments::from_args();
    if args.debug {
        println!("arguments provided:\n{}", args);
    }
    let key = args.key_source.obtain_key()?;
    if args.debug {
        println!("approximate time to crack key: {}", key.time_to_crack());
    }
    if !args.accept_weak_key && !key.is_strong() {
        eprintln!(
            "warning: the encryption key is not strong (it might be cracked in {})",
            key.time_to_crack()
        );
    }
    let config = args.convert(key)?;
    encrypt(&config)
}

#[cfg(test)]
mod tests {
    use ::file_endec::config::typ::EndecConfig;
    use ::file_endec::header::strategy::Verbosity;
    use ::file_endec::key::Key;

    use super::*;

    #[test]
    fn parse_args_minimal() {
        let args = EncryptArguments::from_iter(&["fileenc", "file.txt"]);
        let config = args.convert(Key::new("abcdef123!")).unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert_eq!(config.raw_key().key_data.unsecure(), "abcdef123!");
        assert_eq!(config.verbosity(), Verbosity::Normal);
        assert_eq!(config.overwrite(), false);
        assert_eq!(config.delete_input(), false);
        assert_eq!(config.output_dir(), None);
        assert_eq!(config.output_extension(), ".enc");
        assert_eq!(config.dry_run(), false);
    }

    #[test]
    fn parse_args_long() {
        let args = EncryptArguments::from_iter(&[
            "fileenc",
            "file.txt",
            "-q",
            "-d",
            "-f",
            "-o",
            "/tmp/hello",
            "--output-extension",
            "secret",
            "another_file.txt",
            "there_are_three_files",
        ]);
        let config = args.convert(Key::new("abcdef123!")).unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert!(config.files().contains(&PathBuf::from("another_file.txt")));
        assert!(config.files().contains(&PathBuf::from("there_are_three_files")));
        assert_eq!(3, config.files().len());
        assert_eq!(config.raw_key().key_data.unsecure(), "abcdef123!");
        assert_eq!(config.verbosity(), Verbosity::Quiet);
        assert_eq!(config.overwrite(), true);
        assert_eq!(config.delete_input(), true);
        assert_eq!(
            config.output_dir(),
            Some(PathBuf::from("/tmp/hello").as_path())
        );
        assert_eq!(config.output_extension(), ".secret");
        assert_eq!(config.dry_run(), false);
    }
}
