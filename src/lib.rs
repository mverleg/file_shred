use crate::erase::orchestrate::delete_file;
use crate::util::FedResult;

mod config;
mod erase;
mod util;

pub use ::util::FedResult;
use crate::config::ShredConfig;
use crate::config::conf::ShredConfig;

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
