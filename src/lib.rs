use ::std::path::Path;

use ::indicatif::ProgressBar;

pub use crate::config::conf::ShredConfig;
pub use crate::config::typ::Verbosity;
use crate::erase::orchestrate::delete_file;
use crate::inspect::collect::collect_file_info;
use crate::util::cli::confirm_delete;
pub use crate::util::errors::ShredResult;

mod config;
mod erase;
mod inspect;
mod util;

pub fn shred<P: AsRef<Path>>(config: &ShredConfig<P>) -> ShredResult<()> {
    let files: Vec<_> = config.files.iter().map(|f| (*f).as_ref()).collect();
    let files = collect_file_info(&files, config.verbosity)?;
    let total_kb = files.iter().map(|f| f.size_kb).sum::<u64>() + 10_000;
    let progress = if config.progress_bar {
        Some(ProgressBar::new(total_kb))
    } else {
        None
    };
    if config.confirmation_prompt {
        confirm_delete(&files, config.verbosity.debug())?;
    }
    if let Some(ref pb) = progress {
        pb.inc(10_000);
    }
    for file in &files {
        delete_file(&file.path, config)?;
        if let Some(ref pb) = progress {
            pb.inc(file.size_kb);
        }
    }
    if let Some(ref pb) = progress {
        pb.finish_with_message("done");
    }
    if !config.verbosity.quiet() {
        if config.keep_files {
            println!("removed data from {} files", config.files.len());
        } else {
            println!("shredded and removed {} files", config.files.len());
        }
    }
    Ok(())
}

/// Easy-use wrapper for `shred` that uses defaults for most options and shreds only one file.
pub fn shred_file(path: &Path) -> ShredResult<()> {
    shred(&ShredConfig::non_interactive(
        vec![&path],      // files
        Verbosity::Quiet, // verbosity
        false,            // keep_files
        10,               // overwrite_count
        10,               // rename_count
    ))
}

#[cfg(test)]
mod tests {
    use ::std::fs::File;
    use ::std::io::Read;
    use ::std::io::Write;
    use ::std::path::Path;
    use ::std::path::PathBuf;

    use ::tempfile::tempdir;

    use crate::{shred, ShredConfig};

    use super::*;

    const PREFIX: &[u8] = b"Test file content to be checked afterwards for filename ";

    fn make_file(dir: &Path, name: &str) -> PathBuf {
        let mut pth = dir.to_owned();
        pth.push(name);
        let mut file1 = File::create(&pth).unwrap();
        file1.write_all(PREFIX).unwrap();
        file1.write_all(name.as_bytes()).unwrap();
        pth
    }

    fn read_file(pth: &Path) -> Vec<u8> {
        let mut data = vec![];
        File::open(&pth).unwrap().read_to_end(&mut data).unwrap();
        data
    }

    #[test]
    fn demo() {
        let dir = tempdir().unwrap();
        let pth1 = make_file(dir.path(), "file_1.txt");
        let pth2 = make_file(dir.path(), "other_file.bye");
        let mut config = ShredConfig::non_interactive(
            vec![&pth1, &pth2], // files
            Verbosity::Debug,   // verbosity
            true,               // keep_files
            6,                  // overwrite_count
            3,                  // rename_count
        );
        assert!(pth1.exists());
        assert!(pth2.exists());

        // Overwrite but don't delete
        shred(&config).unwrap();
        assert!(pth1.exists());
        assert!(pth2.exists());
        let data1 = read_file(&pth1);
        assert!(!data1.starts_with(PREFIX));
        assert!(!data1.ends_with(b"file_1.txt"));
        let data2 = read_file(&pth2);
        assert!(!data2.starts_with(PREFIX));
        assert!(!data2.ends_with(b"other_file.bye"));

        // Delete
        config.keep_files = false;
        shred(&config).unwrap();
        assert!(!pth1.exists());
        assert!(!pth2.exists());
    }

    #[test]
    fn owned_paths() {
        let dir = tempdir().unwrap();
        let pth1 = make_file(dir.path(), "file_1.txt");
        let pth2 = make_file(dir.path(), "other_file.bye");
        let config = ShredConfig::<PathBuf>::non_interactive(
            vec![pth1.clone(), pth2.clone()], // files
            Verbosity::Debug,                 // verbosity
            false,                            // keep_files
            6,                                // overwrite_count
            3,                                // rename_count
        );
        assert!(pth1.exists());
        assert!(pth2.exists());
        shred(&config).unwrap();
        assert!(!pth1.exists());
        assert!(!pth2.exists());
    }

    #[test]
    fn test_shred_file() {
        let dir = tempdir().unwrap();
        let pth1 = make_file(dir.path(), "shred.me");
        shred_file(&pth1).unwrap();
        assert!(!pth1.exists());
    }
}
