use ::file_endec::EncryptConfig;
use ::structopt::StructOpt;
use ::std::path::PathBuf;
use ::std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "FileEnc", about = "Securely encrypt one or more files using the given key.")]
pub struct EncryptArguments {

    #[structopt(name = "FILES", parse(from_os_str), required = true, help = "One or more paths to input files (absolute or relative)")]
    files: Vec<PathBuf>,

    #[structopt(short = "k", long, help = "The encryption key, for batch use. It is generally safer to not pass this and be prompted for it instead.")]
    key: Option<String>,

    #[structopt(short = "v", long, help = "Show debug information, especially on errors.")]
    debug: bool,

    #[structopt(short = "f", long, help = "Overwrite output files if they exist.")]
    overwrite: bool,

    #[structopt(short = "d", long, help = "Delete input files after successful encryption (overwrites garbage before delete).")]
    delete_input: bool,

    #[structopt(parse(from_os_str), short = "o", long, help = "Alternative output directory. If not given, output is saved alongside input.")]
    output_dir: Option<PathBuf>,

    #[structopt(short = "e", long, default_value = ".enc", help = "Extension added to encrypted files.")]
    output_extension: String,

    #[structopt(long, help = "Test encryption, but do not save encrypted files (nor delete input, if --delete-input).")]
    dry_run: bool,
}

impl From<EncryptArguments> for EncryptConfig {
    fn from(args: EncryptArguments) -> Self {
        assert!(args.key.is_some(), "key must be set before calling 'from'/'into'");
        EncryptConfig::new(
            args.files,
            args.key.unwrap(),
            args.debug,
            args.overwrite,
            args.delete_input,
            args.output_dir,
            args.output_extension,
            args.dry_run,
        )
    }
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
            },
            None => f.write_str("  output is stored alongside input")?,
        }
        f.write_str("\n")?;

        f.write_str("  extension: ")?;
        //TODO @mark: absolute path?
        f.write_str(&self.output_extension)?;
        f.write_str("\n")?;

        // Currently, this is always "on", because printing is only used in debug mode.
        f.write_str("  debug logging: ")?;
        f.write_str(if self.debug { "on" } else { "off" })?;
        f.write_str("\n")?;

        f.write_str("  dry run: ")?;
        f.write_str(if self.dry_run { "yes" } else { "no" })?;
        f.write_str("\n")?;

        f.write_str("  overwrite existing output files: ")?;
        f.write_str(if self.overwrite { if self.dry_run { "no (overridden by dry run)" } else { "yes" }} else { "no" })?;
        f.write_str("\n")?;

        f.write_str("  delete input files: ")?;
        f.write_str(if self.delete_input { if self.dry_run { "no (overridden by dry run)" } else { "yes" }} else { "no" })?;
        f.write_str("\n")?;

        Ok(())
    }
}

pub fn main() {
    let args = EncryptArguments::from_args();
    if args.debug {
        println!("arguments provided:\n{}", args);
    }
}
