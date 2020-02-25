use ::std::fs;
use ::std::fs::File;
use ::std::io::Write;

use crate::config::enc::EncryptConfig;
use crate::config::typ::EndecConfig;
use crate::config::DecryptConfig;
use crate::files::file_meta::FileInfo;
use crate::header::write_header;
use crate::header::Header;
use crate::util::errors::wrap_io;
use crate::util::FedResult;

pub fn write_output_file(
    config: &EncryptConfig,
    file: &FileInfo,
    secret: &Vec<u8>,
    header: &Header,
) -> FedResult<()> {
    assert!(!config.dry_run());
    if file.out_pth.exists() {
        if config.overwrite() {
            assert!(file.out_pth.is_file());
            fs::remove_file(&file.out_pth).map_err(|_| {
                format!("Failed to remove previously-existing file that exists in output location")
            })?;
        } else {
            return Err(format!(
                "While encrypting, a file appeared in previously empty \
                        output location '{}'",
                &file.out_pth.to_string_lossy()
            ));
        }
    }
    let mut out_file = wrap_io(
        || {
            format!(
                "Could not create output file for '{}'",
                &file.out_pth.to_string_lossy()
            )
        },
        File::create(&file.out_pth),
    )?;
    write_header(&mut out_file, &header, config.debug())?;
    wrap_io(
        || {
            format!(
                "Failed to write encrypted output data for '{}'",
                &file.out_pth.to_string_lossy()
            )
        },
        out_file.write_all(&secret),
    )?;
    if config.debug() {
        println!("encrypted {}", &file.out_pth.to_string_lossy());
    }
    Ok(())
}
