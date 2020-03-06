use ::std::fmt;
use ::std::path::PathBuf;
use ::std::process::exit;

use ::structopt::StructOpt;

use ::file_endec::header::strategy::Verbosity;
use ::file_endec::key::Key;
use ::file_endec::key::KeySource;
use ::file_endec::util::FedResult;
use ::file_endec::config::DecryptConfig;
use ::file_endec::decrypt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "FileEnc",
author = "github.com/mverleg/file_endec",
about = "Securely encrypt one or more files using the given key."
)]
pub struct DecryptArguments {
    #[structopt(
    name = "FILES",
    parse(from_os_str),
    required = true,
    min_values = 1,
    help = "One or more paths to encrypted input files (absolute or relative)"
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
    help = "Delete encrypted input files after successful decryption."
    )]
    delete_input: bool,

    #[structopt(
    parse(from_os_str),
    short = "o",
    long,
    help = "Alternative output directory. If not given, output is saved alongside input."
    )]
    output_dir: Option<PathBuf>,
}

impl fmt::Display for DecryptArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("  files:\n")?;
        for file in self.files.clone().into_iter() {
            f.write_str("  - ")?;
            f.write_str(file.to_string_lossy().as_ref())?;
            f.write_str("\n")?;
        }

        match &self.output_dir {
            Some(dir) => {
                f.write_str("  output directory: ")?;
                f.write_str(dir.to_string_lossy().as_ref())?
            }
            None => f.write_str("  output is stored alongside input")?,
        }
        f.write_str("\n")?;

        // Currently, this is always "on", because printing is only used in debug mode.
        f.write_str("  debug logging: ")?;
        f.write_str(if self.debug { "on" } else { "off" })?;
        f.write_str("\n")?;

        f.write_str("  overwrite existing output files: ")?;
        f.write_str(if self.overwrite {
            "yes"
        } else {
            "no"
        })?;
        f.write_str("\n")?;

        f.write_str("  delete input files: ")?;
        f.write_str(if self.delete_input {
            "yes"
        } else {
            "no"
        })?;

        Ok(())
    }
}

pub fn main() {
    if let Err(err) = go() {
        eprintln!("{}", err);
        exit(1);
    }
}

impl DecryptArguments {
    fn convert(self, key: Key) -> FedResult<DecryptConfig> {
        let verbosity = match (self.debug, self.quiet) {
            (true, true) => return Err("cannot use quiet mode and debug mode together".to_owned()),
            (true, false) => Verbosity::Debug,
            (false, true) => Verbosity::Quiet,
            (false, false) => Verbosity::Normal,
        };
        Ok(DecryptConfig::new(
            self.files,  // files
            key,  // raw_key
            verbosity,  // verbosity
            self.overwrite,  // overwrite
            self.delete_input,  // delete_input
            self.output_dir,  // output_dir
        ))
    }
}

//TODO: if wildcards or directories are ever supported, then skip files that have the encrypted extension (i.e. .enc)

fn go() -> FedResult<()> {
    let args = DecryptArguments::from_args();
    if args.debug {
        println!("arguments provided:\n{}", args);
    }
    let key = args.key_source.obtain_key()?;
    if args.debug {
        println!("approximate time to crack key: {}", key.time_to_crack());
    }
    let config = args.convert(key)?;
    decrypt(&config)
}

#[cfg(test)]
mod tests {
    use ::file_endec::header::strategy::Verbosity;
    use ::file_endec::key::Key;
    use ::file_endec::config::typ::EndecConfig;

    use super::*;

    #[test]
    fn parse_args_minimal() {
        let args = DecryptArguments::from_iter(&["fileenc", "file.txt"]);
        let config = args.convert(Key::new("abcdef123!")).unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert_eq!(config.raw_key().key_data.unsecure(), "abcdef123!");
        assert_eq!(config.verbosity(), Verbosity::Normal);
        assert_eq!(config.overwrite(), false);
        assert_eq!(config.delete_input(), false);
        assert_eq!(config.output_dir(), None);
    }

    #[test]
    fn parse_args_long() {
        let args = DecryptArguments::from_iter(&[
            "fileenc",
            "file.txt",
            "-q",
            "-d",
            "-f",
            "-o",
            "/tmp/hello",
        ]);
        let config = args.convert(Key::new("abcdef123!")).unwrap();
        assert!(config.files().contains(&PathBuf::from("file.txt")));
        assert_eq!(config.raw_key().key_data.unsecure(), "abcdef123!");
        assert_eq!(config.verbosity(), Verbosity::Quiet);
        assert_eq!(config.overwrite(), true);
        assert_eq!(config.delete_input(), true);
        assert_eq!(
            config.output_dir(),
            Some(PathBuf::from("/tmp/hello").as_path())
        );
    }
}
