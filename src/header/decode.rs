use ::std::io::BufRead;

use ::semver::Version;

use crate::files::Checksum;
use crate::header::Header;
use crate::header::HEADER_CHECKSUM_MARKER;
use crate::header::HEADER_DATA_MARKER;
use crate::header::HEADER_MARKER;
use crate::header::HEADER_SALT_MARKER;
use crate::header::HEADER_VERSION_MARKER;
use crate::key::salt::Salt;
use crate::util::FedResult;

fn read_line(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<()> {
    line.clear();
    let res = reader.read_line(line);
    if let Err(err) = res {
        return Err(if verbose {
            "could not read file".to_owned()
        } else {
            format!("could not read file (error: {})", err)
        });
    }
    line.pop();
    Ok(())
}

fn check_prefix<'a>(line: &'a str, prefix: &str, verbose: bool) -> FedResult<&'a str> {
    if prefix.len() <= line.len() && &line[..prefix.len()] == prefix {
        Ok(&line[prefix.len()..])
    } else {
        Err(if verbose {
            "encryption header was incorrect".to_owned()
        } else {
            format!(
                "encryption header was incorrect (expected '{}', but it was not found)",
                prefix
            )
        })
    }
}

fn parse_marker(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<()> {
    read_line(reader, line, verbose)?;
    if HEADER_MARKER != line {
        return Err(if verbose {
            format!("did not recognize encryption header (expected '{}', got '{}'); was this file really encrypted with fileenc?", HEADER_MARKER, line)
        } else {
            "did not recognize encryption header; was this file really encrypted with fileenc?"
                .to_owned()
        });
    }
    Ok(())
}

fn parse_version(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<Version> {
    read_line(reader, line, verbose)?;
    let verion_str = check_prefix(line, HEADER_VERSION_MARKER, verbose)?;
    match Version::parse(verion_str) {
        Ok(version) => Ok(version),
        Err(err) => Err(if verbose {
            format!("could not determine the version of fileenc that encrypted this file; got {} which is invalid, reason: {}", verion_str, err)
        } else {
            "could not determine the version of fileenc that encrypted this file".to_owned()
        }),
    }
}

fn parse_salt(reader: &mut dyn BufRead, line: &mut String, verbose: bool) -> FedResult<Salt> {
    read_line(reader, line, verbose)?;
    let salt_str = check_prefix(line, HEADER_SALT_MARKER, verbose)?;
    Salt::parse_base64(salt_str)
}

fn parse_checksum(
    reader: &mut dyn BufRead,
    line: &mut String,
    verbose: bool,
) -> FedResult<Checksum> {
    read_line(reader, line, verbose)?;
    let checksum_str = check_prefix(line, HEADER_CHECKSUM_MARKER, verbose)?;
    Checksum::parse(checksum_str)
}

pub fn parse_header(reader: &mut dyn BufRead, verbose: bool) -> FedResult<Header> {
    let mut line = String::new();
    parse_marker(reader, &mut line, verbose)?;
    let version = parse_version(reader, &mut line, verbose)?;
    let salt = parse_salt(reader, &mut line, verbose)?;
    let checksum = parse_checksum(reader, &mut line, verbose)?;
    read_line(reader, &mut line, verbose)?;
    check_prefix(&line, HEADER_DATA_MARKER, verbose).unwrap();
    Header::new(version, salt, checksum, verbose)
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use crate::files::Checksum;
    use crate::header::Header;
    use crate::key::salt::Salt;

    use super::parse_header;

    #[test]
    fn read_v1_0_0_one() {
        let version = Version::parse("1.0.0").unwrap();
        let input =
            "github.com/mverleg/file_endec\nv 1.0.0\nsalt AQAAAAAAAAA\ncheck xx_sha256 Ag\ndata:\n";
        let expected = Header::new(
            version,
            Salt::new(1),
            Checksum::fixed_for_test(vec![2]),
            true,
        )
        .unwrap();
        let mut buf = input.as_bytes();
        let header = parse_header(&mut buf, true).unwrap();
        assert_eq!(expected, header);
    }

    #[test]
    fn read_v1_0_0_two() {
        let version = Version::parse("1.0.0").unwrap();
        let input = "github.com/mverleg/file_endec\nv 1.0.0\nsalt Fc1bBwAAAAA\ncheck xx_sha256 AAUABQAFAAUABQAF\ndata:\n";
        let expected = Header::new(
            version,
            Salt::new(123_456_789),
            Checksum::fixed_for_test(vec![0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5]),
            true,
        )
        .unwrap();
        let mut buf = input.as_bytes();
        let header = parse_header(&mut buf, true).unwrap();
        assert_eq!(expected, header);
    }
}
