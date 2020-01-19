use ::std::io::BufRead;

use ::semver::Version;

use crate::strategy::{Strategy, get_version_strategy};
use crate::util::FedResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    version: Version,
    salt: u64,
    strategy: &'static Strategy,
    checksum: u64,
}

const HEADER_MARKER: &str = "github.com/mverleg/file_endec";

pub fn write_header() {
    unimplemented!()
}

fn read_line(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<()> {
    line.clear();
    if let Err(err) = reader.read_line(line) {
        //TODO @mark: verbose logging
        return Err(match verbose {
            true => "could not read file".to_owned(),
            false => format!("could not read file (error: {})", err),
        });
    }
    Ok(())
}

fn check_prefix<'a>(line: &'a str, prefix: &str, verbose: bool) -> FedResult<&'a str> {
    match prefix.len() <= line.len() && &line[..prefix.len()] == prefix {
        true => Ok(&line[prefix.len()..]),
        false => Err(match verbose {
            true => "encryption header was incorrect".to_owned(),
            false => format!("encryption header was incorrect (expected '{}', but it was not found)", prefix),
        }),
    }
}

fn parse_marker(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<()> {
    read_line(reader, line, verbose)?;
    if HEADER_MARKER != line {
        return Err(match verbose {
            true => "did not recognize encryption header; was this file really encrypted with fileenc?".to_owned(),
            false => format!("did not recognize encryption header (expected '{}', got '{}'); was this file really encrypted with fileenc?", HEADER_MARKER, line),
        });
    }
    Ok(())
}

fn parse_version(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<Version> {
    read_line(reader, line, verbose)?;
    let verion_str = check_prefix(line, "v ", verbose)?;
    match Version::parse(verion_str) {
        Ok(version) => Ok(version),
        Err(err) => Err(match verbose {
            true => "could not determine the version of fileenc that encrypted this file".to_owned(),
            false => format!("could not determine the version of fileenc that encrypted this file; \
            got {} which is invalid, reason: {}", verion_str, err),
        }),
    }
}

fn parse_salt(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<u64> {
    read_line(reader, line, verbose)?;
    let salt_str = check_prefix(line, "salt ", verbose)?;
    match u64::from_str_radix(salt_str, 32) {
        Ok(salt) => Ok(salt),
        Err(err) => Err(match verbose {
            true => "could not determine the salt used by fileenc that encrypted this file".to_owned(),
            false => format!("could not determine the salt used by fileenc that encrypted this file; \
            got {} which is invalid, reason: {}", salt_str, err),
        }),
    }
}

fn parse_checksum(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<u64> {
    read_line(reader, line, verbose)?;
    let checksum_str = check_prefix(line, "check ", verbose)?;
    match u64::from_str_radix(checksum_str, 32) {
        Ok(version) => Ok(version),
        Err(err) => Err(match verbose {
            true => "could not determine the checksum of the encrypted file".to_owned(),
            false => format!("could not determine the checksum of the encrypted file; \
            got {} which is invalid, reason: {}", checksum_str, err),
        }),
    }
}

pub fn parse_header(reader: &mut dyn BufRead, verbose: bool) -> FedResult<Header> {
    let mut line = String::new();
    parse_marker(reader, &mut line, verbose)?;
    let version = parse_version(reader, &mut line, verbose)?;
    let strategy = get_version_strategy(&version, verbose)
        .map_err(|e| format!("version used to encrypt: {}", e))?;
    let salt = parse_salt(reader, &mut line, verbose)?;
    let checksum = parse_checksum(reader, &mut line, verbose)?;
    Ok(Header {
        version,
        salt,
        strategy,
        checksum,
    })
}

