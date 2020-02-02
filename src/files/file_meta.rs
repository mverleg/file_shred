use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::util::FedResult;

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub path: &'a Path,
    pub size_kb: u64,
    //TODO @mark: make sure encrypted file has same permissions and owner as original
    pub permissions: (),
}

impl <'a> FileInfo<'a> {
    pub fn path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

pub fn inspect_files(files: &[PathBuf], verbose: bool) -> FedResult<Vec<FileInfo>> {
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
            permissions: (),
        });
    }
    if not_found_cnt > 0 {
        return Err(format!("aborting because {} input file{} not found", not_found_cnt,
                           if not_found_cnt > 1 { "s were" } else { " was" }));
    }
    Ok(infos)
}
