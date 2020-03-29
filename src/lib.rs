use crate::erase::orchestrate::delete_file;

pub use crate::config::conf::ShredConfig;
pub use crate::config::typ::Verbosity;
pub use crate::util::errors::ShredResult;

mod config;
mod erase;
mod util;

pub fn shred(config: &ShredConfig) -> ShredResult<()> {
    for file in &config.files {
        delete_file(&file, config)?;
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
