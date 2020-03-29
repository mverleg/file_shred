pub use crate::config::conf::ShredConfig;
pub use crate::config::typ::Verbosity;
use crate::erase::orchestrate::delete_file;
use crate::inspect::collect::collect_file_info;
use crate::util::cli::confirm_delete;
pub use crate::util::errors::ShredResult;

mod config;
mod inspect;
mod erase;
mod util;

pub fn shred(config: &ShredConfig) -> ShredResult<()> {
    let files = collect_file_info(config.files.clone(), &config.verbosity)?;
    if config.confirmation_prompt {
        confirm_delete(&files, config.verbosity.debug())?;
    }
    for file in &files {
        delete_file(&file.path, config)?;
    }
    if !config.verbosity.quiet() {
        println!("shredded {} files", config.files.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::fs::File;
    use ::std::io::Write;
    use ::std::path::Path;
    use ::std::path::PathBuf;

    use ::tempfile::tempdir;
    use crate::{shred, ShredConfig};

    fn make_file(dir: &Path, name: &str) -> PathBuf {
        let mut pth = dir.to_owned();
        pth.push(name);
        let mut file1 = File::create(&pth).unwrap();
        file1.write_all(b"Test file content for ").unwrap();
        file1.write_all(name.as_bytes()).unwrap();
        return pth;
    }

    #[test]
    fn demo() {
        let dir = tempdir().unwrap();
        let pth1 = make_file(dir.path(), "file_1.txt");
        let pth2 = make_file(dir.path(), "other_file.bye");
        let config = ShredConfig::new(
            vec![pth1, pth2],  // files
            false,  // confirmation_prompt
            Verbosity::Debug,  // verbosity
            false,  // keep_files
            6, // overwrite_count
            3, // rename_count
        );
        shred(&config).unwrap()
    }
}
