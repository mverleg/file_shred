use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::enc::EncryptConfig;
use crate::util::FedResult;

pub mod util;
pub mod header;
pub mod key;
pub mod config;

struct FileInfo<'a> {
    path: &'a Path,
    size_kb: u64,
}

fn inspect_files<'a>(files: &'a [PathBuf], verbose: bool) -> FedResult<Vec<FileInfo>> {
    let mut not_found_cnt = 0;
    let mut infos = Vec::with_capacity(files.len());
    for file in files {
        let meta = match fs::metadata(file) {
            Ok(meta) => meta,
            Err(err) => {
                match verbose {
                    true => eprintln!("could not read file '{}'; reason: {}",
                                      file.to_string_lossy(), err),
                    false => eprintln!("could not read file '{}'",
                                       file.to_string_lossy()),
                }
                not_found_cnt += 1;
                continue;
            }
        };
        if !meta.is_file() {
            eprintln!("path '{}' is not a file", file.to_string_lossy());
            not_found_cnt += 1;
            continue;
        }
        infos.push(FileInfo {
            path: file.as_path(),
            size_kb: meta.len() / 1024,
        });
    }
    if not_found_cnt > 0 {
        return Err(format!("aborting because {} input file{} not found", not_found_cnt,
                           if not_found_cnt > 1 { "s were" } else { " was" }));
    }
    Ok(infos)
}

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    let files_info = inspect_files(config.files(), config.debug())?;

    unimplemented!()
}

pub fn decrypt(_config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}
