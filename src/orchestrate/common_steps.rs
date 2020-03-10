

use crate::files::file_meta::FileInfo;
use crate::header::HEADER_MARKER;
use crate::header::strategy::Verbosity;
use crate::util::errors::wrap_io;
use crate::util::FedResult;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn open_reader(file: &FileInfo, verbosity: Verbosity) -> FedResult<BufReader<File>> {
    match File::open(file.in_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(err) => Err(if verbosity.debug() {
            format!("could not open input file {}: {}", file.path_str(), err)
        } else {
            format!("could not open input file {}", file.path_str())
        }),
    }
}

pub fn read_file(reader: &mut BufReader<File>, path_str: &str, size_kb: u64, verbosity: Verbosity) -> FedResult<Vec<u8>> {
    if verbosity.debug() {
        println!("reading {}", path_str);
    }
    if !verbosity.quiet() && size_kb > 1024 * 1024 {
        eprintln!(
            "warning: reading {} Mb file '{}' into RAM",
            size_kb / 1024,
            path_str
        );
    }
    let mut data = vec![];
    wrap_io(|| "could not read input file", reader.read_to_end(&mut data))?;
    if !verbosity.quiet() && data.starts_with(HEADER_MARKER.as_bytes()) {
        eprintln!(
            "warning: file '{}' seems to already be encrypted",
            path_str
        );
    }
    Ok(data)
}
