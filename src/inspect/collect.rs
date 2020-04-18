use crate::{ShredResult, Verbosity};
use std::path::Path;
use std::{fmt, fs};

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub path: &'a Path,
    pub size_kb: u64,
}

impl <'a> fmt::Display for &'a FileInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} kb)", self.path.to_string_lossy(), self.size_kb)
    }
}

pub fn collect_file_info<'a>(files: &'a [&'a Path], verbosity: Verbosity) -> ShredResult<Vec<FileInfo<'a>>> {
    let mut infos = Vec::with_capacity(files.len());
    let mut not_found_cnt: u32 = 0;
    for file in files.into_iter() {
        // Input file
        let meta = match fs::metadata(file) {
            Ok(meta) => meta,
            Err(err) => {
                if verbosity.debug() {
                    eprintln!(
                        "could not read file '{}'; reason: {}",
                        &file.to_string_lossy(),
                        err
                    )
                } else {
                    eprintln!("could not read file '{}'", &file.to_string_lossy())
                }
                not_found_cnt += 1;
                continue;
            }
        };
        if !meta.is_file() {
            eprintln!("path '{}' is not a file", &file.to_string_lossy());
            not_found_cnt += 1;
            continue;
        }

        infos.push(FileInfo {
            path: file,
            size_kb: (meta.len() + 1023) / 1024,
        });
    }
    if not_found_cnt > 0 {
        return Err(format!(
            "aborting because {} input file{} not found",
            not_found_cnt,
            if not_found_cnt > 1 { "s were" } else { " was" }
        ));
    }
    Ok(infos)
}
