use ::file_endec::EncryptConfig;
use ::structopt::StructOpt;
use std::path::PathBuf;

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

    #[structopt(parse(from_os_str), short = "o", long, default_value = ".", help = "Alternative output directory.")]
    output_dir: PathBuf,

    #[structopt(short = "e", long, default_value = ".enc", help = "Extension added to encrypted files.")]
    output_extension: String,

    #[structopt(long, help = "Test encryption, but do not save encrypted files (nor delete input, if --delete-input).")]
    dry_run: bool,
}

impl From<EncryptArguments> for EncryptConfig {
    fn from(args: EncryptArguments) -> Self {
        unimplemented!()
    }
}

pub fn main() {
    let args = EncryptArguments::from_args();
    if args.files.len() == 0 {

    }
    println!("{:?}", args);
}
