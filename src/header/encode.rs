use ::std::error::Error;
use ::std::io::Write;

use ::semver::Version;

use crate::header::Checksum;
use crate::header::Header;
use crate::header::HEADER_CHECKSUM_MARKER;
use crate::header::HEADER_DATA_MARKER;
use crate::header::HEADER_MARKER;
use crate::header::HEADER_SALT_MARKER;
use crate::header::HEADER_VERSION_MARKER;
use crate::header::Salt;
use crate::util::FedResult;
use crate::util::util::u64_to_base64str;

fn wrap_err(res: Result<usize, impl Error>, verbose: bool) -> FedResult<()> {
    if let Err(err) = res {
        Err(match verbose {
            true => "failed to write encryption header".to_owned(),
            false => format!("failed to write encryption header, reason: {}", err),
        })
    } else {
        Ok(())
    }
}

fn write_line(writer: &mut impl Write, prefix: &str, value: Option<String>, verbose: bool) -> FedResult<()> {
    wrap_err(writer.write(prefix.as_bytes()), verbose)?;
    if let Some(text) = value {
        wrap_err(writer.write(text.as_bytes()), verbose)?;
    }
    wrap_err(writer.write("\n".as_bytes()), verbose)?;
    Ok(())
}

fn write_marker(writer: &mut impl Write, verbose: bool) -> FedResult<()> {
    write_line(writer, HEADER_MARKER, None, verbose)
}

fn write_version(writer: &mut impl Write, version: &Version, verbose: bool) -> FedResult<()> {
    let version_str = format!("{}.{}.{}", version.major, version.minor, version.patch);
    write_line(writer, HEADER_VERSION_MARKER, Some(version_str), verbose)
}

fn write_salt(writer: &mut impl Write, salt: &Salt, verbose: bool) -> FedResult<()> {
    let salt_str = u64_to_base64str(salt.as_primitive());
    write_line(writer, HEADER_SALT_MARKER, Some(salt_str), verbose)
}

fn write_checksum(writer: &mut impl Write, checksum: &Checksum, verbose: bool) -> FedResult<()> {
    let checksum_str = u64_to_base64str(checksum.as_primitive());
    write_line(writer, HEADER_CHECKSUM_MARKER, Some(checksum_str), verbose)
}

pub fn write_header(writer: &mut impl Write, header: &Header, verbose: bool) -> FedResult<()> {
    write_marker(writer, verbose)?;
    write_version(writer, header.version(), verbose)?;
    write_salt(writer, header.salt(), verbose)?;
    write_checksum(writer, header.checksum(), verbose)?;
    write_line(writer, HEADER_DATA_MARKER, None, verbose)?;
    Ok(())
}

#[cfg(test)]
mod header {
    use super::write_header;
    use crate::header::{Header, Salt, Checksum};
    use semver::Version;
    use std::str::from_utf8;

    #[test]
    fn write_v1_0_0() {
        let version = Version::parse("1.0.0").unwrap();
        let header = Header::new(
            version,
            Salt::new(123_456_789),
            Checksum::new(050_505_050_505),
            &true,
        ).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        write_header(&mut buf, &header, true).unwrap();
        let expected = "github.com/mverleg/file_endec\nv 1.0.0\nsalt Fc1bBwAAAAA\ncheck ielVwgsAAAA\ndata:\n";
        assert_eq!(expected, from_utf8(&buf).unwrap());
    }
}
