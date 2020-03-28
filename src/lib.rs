pub use crate::config::conf::ShredConfig;
use crate::erase::orchestrate::delete_file;
pub use crate::util::errors::FedResult;

mod config;
mod erase;
mod util;

pub fn shred(config: &ShredConfig) -> FedResult<()> {
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
