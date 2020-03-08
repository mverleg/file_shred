use ::std::fs;

use crate::files::file_meta::FileInfo;
use crate::header::HEADER_MARKER;
use crate::header::strategy::Verbosity;
use crate::util::errors::wrap_io;
use crate::util::FedResult;

pub fn read_file(file: &FileInfo, verbosity: &Verbosity) -> FedResult<Vec<u8>> {
    if verbosity.debug() {
        println!("encrypting {}", file.path_str());
    }
    if !verbosity.quiet() && file.size_kb > 1024 * 1024 {
        eprintln!(
            "warning: reading {} Mb file '{}' into RAM",
            file.size_kb / 1024,
            file.path_str()
        );
    }
    let data = wrap_io(|| "could not read input file", fs::read(file.in_path))?;
    if !verbosity.quiet() && data.starts_with(HEADER_MARKER.as_bytes()) {
        eprintln!(
            "warning: file '{}' seems to already be encrypted",
            file.path_str()
        );
    }
    Ok(data)
}
