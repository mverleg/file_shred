use crate::erase::orchestrate::delete_file;
use crate::util::FedResult;

mod config;
mod erase;
mod util;

pub fn shred(config: &ShredConfig) -> FedResult<()> {
    let files
    let _total_size_kb: u64 = files_info.iter().map(|inf| inf.size_kb).sum();
    let salt = Salt::generate_random()?;
    let stretched_key = stretch_key(
        config.raw_key(),
        &salt,
        strategy.stretch_count,
        &strategy.key_hash_algorithms,
    );
    //TODO @mark: progress logging
    for file in &files_info {
        let mut reader = open_reader(&file, config.verbosity())?;
        let data = read_file(&mut reader, &file.path_str(), file.size_kb, config.verbosity())?;
        let checksum = calculate_checksum(&data);
        let small = compress_file(data, &strategy.compression_algorithm)?;
        let secret = encrypt_file(small, &stretched_key, &salt, &strategy.symmetric_algorithms);
        let header = Header::new(version.clone(), salt.clone(), checksum, config.debug())?;
        if !config.dry_run() {
            write_output_file(config, &file, &secret, Some(&header))?;
        } else if !config.quiet() {
            println!(
                "successfully encrypted '{}' ({} kb); not saving to '{}' because of dry-run",
                file.path_str(),
                secret.len() / 1024,
                &file.out_pth.to_string_lossy(),
            );
        }
    }
    if !config.quiet() {
        println!("encrypted {} files", files_info.len());
    }
    Ok(())



    delete_file();
    unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn demo() {
        unimplemented!()
    }
}
