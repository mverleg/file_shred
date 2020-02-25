use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::EndecConfig;
use crate::util::pth::determine_output_path;
use crate::util::FedResult;

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub in_path: &'a Path,
    pub size_kb: u64,
    pub out_pth: PathBuf,
    //TODO: make sure encrypted file has same permissions and owner as original
    pub permissions: (),
}

impl<'a> FileInfo<'a> {
    pub fn path_str(&self) -> String {
        self.in_path.to_string_lossy().to_string()
    }
}

pub fn inspect_files<'a>(
    files: &'a [PathBuf],
    config: &impl EndecConfig,
) -> FedResult<Vec<FileInfo<'a>>> {
    let mut not_found_cnt: u32 = 0;
    let mut output_exists_cnt: u32 = 0;
    let mut infos = Vec::with_capacity(files.len());
    for file in files {
        // Input file
        let meta = match fs::metadata(file) {
            Ok(meta) => meta,
            Err(err) => {
                if config.debug() {
                    eprintln!(
                        "could not read file '{}'; reason: {}",
                        file.to_string_lossy(),
                        err
                    )
                } else {
                    eprintln!("could not read file '{}'", file.to_string_lossy())
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

        // Output file
        let output_file =
            determine_output_path(file.as_path(), config.extension(), config.output_dir());
        if !config.overwrite() && output_file.exists() {
            eprintln!("path '{}' is not a file", file.to_string_lossy());
            output_exists_cnt += 1;
        }

        infos.push(FileInfo {
            in_path: file.as_path(),
            size_kb: meta.len() / 1024,
            out_pth: output_file,
            permissions: (),
        });
    }
    if not_found_cnt > 0 {
        return Err(format!(
            "aborting because {} input file{} not found",
            not_found_cnt,
            if not_found_cnt > 1 { "s were" } else { " was" }
        ));
    } else if output_exists_cnt > 0 {
        return Err(format!(
            "aborting because {} output file{} already exist (use --overwrite to overwrite, or --output-dir or -- output-extension to control output location)",
            not_found_cnt,
            if not_found_cnt > 1 { "s" } else { "" }
        ));
    }
    Ok(infos)
}
