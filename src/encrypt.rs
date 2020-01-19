use ::file_endec::EncryptConfig;
use ::structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "FileEnc", about = "Securely encrypt one or more files using the given key.")]
pub struct EncryptArguments {

    //TODO @mark: at least 1
    #[structopt(name = "FILES", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(short = "k", long)]
    key: Option<String>,

    #[structopt(short = "v", long)]
    debug: bool,

    #[structopt(short = "f", long)]
    overwrite: bool,

    #[structopt(short = "d", long)]
    delete_input: bool,

    #[structopt(parse(from_os_str), short = "o", long)]
    output_dir: Option<PathBuf>,

    #[structopt(short = "e", long)]
    output_extension: Option<String>,
}

impl From<EncryptArguments> for EncryptConfig {
    fn from(args: EncryptArguments) -> Self {
        unimplemented!()
    }
}

pub fn main() {
    let args = EncryptArguments::from_args();
    println!("{:?}", args);
}
