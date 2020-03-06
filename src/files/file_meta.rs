use ::std::fs;
use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::EndecConfig;
use crate::config::EncryptConfig;
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

pub fn inspect_files(config: &EncryptConfig) -> FedResult<Vec<FileInfo>> {
    let mut not_found_cnt: u32 = 0;
    let mut output_exists_cnt: u32 = 0;
    let mut infos = Vec::with_capacity(config.files().len());
    for file in config.files() {
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
        let output_file = determine_output_path(
            file.as_path(),
            config.output_extension(),
            config.output_dir(),
        );
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

#[cfg(test)]
mod tests {
    use ::tempfile::NamedTempFile;
    use ::tempfile::TempDir;

    use crate::header::strategy::Verbosity;
    use crate::key::Key;

    use super::*;

    #[test]
    fn output_path() {
        let pth = TempDir::new().unwrap();
        let in_file_1 = NamedTempFile::new_in(pth.path()).unwrap();
        let in_file_2 = NamedTempFile::new_in(pth.path()).unwrap();
        let conf = EncryptConfig::new(
            vec![in_file_1.path().to_owned(), in_file_2.path().to_owned()],
            Key::new("secret"),
            Verbosity::Debug,
            true,
            true,
            None,
            ".enc".to_owned(),
            false,
        );
        let out_files = inspect_files(&conf).unwrap();
        assert_eq!(2, out_files.len());
        let expected_out_pth_1 = format!("{}.enc", in_file_1.path().to_str().unwrap());
        let expected_out_pth_2 = format!("{}.enc", in_file_2.path().to_str().unwrap());
        assert_eq!(out_files[0].out_pth.to_string_lossy(), expected_out_pth_1);
        assert_eq!(out_files[1].out_pth.to_string_lossy(), expected_out_pth_2);
    }

    //TODO @mark: test for overwrite output?
}
