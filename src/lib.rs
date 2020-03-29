use crate::erase::orchestrate::delete_file;

pub use crate::config::conf::ShredConfig;
pub use crate::config::typ::Verbosity;
pub use crate::util::errors::ShredResult;
use crate::inspect::collect::collect_file_info;
use crate::util::cli::{confirmation_prompt, confirm_delete};

mod config;
mod inspect;
mod erase;
mod util;

pub fn shred(config: &ShredConfig) -> ShredResult<()> {
    let files = collect_file_info(config.files.clone(), &config.verbosity)?;
    if config.confirmation_prompt {
        confirm_delete(&files)?;
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

    #[test]
    fn demo() {
        unimplemented!()
    }
}
