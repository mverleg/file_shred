use ::std::fs;
use ::std::fs::OpenOptions;
use ::std::io::{Seek, SeekFrom, Write};
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::rc::Rc;

use ::rand::RngCore;

use crate::util::base64::u64_to_base64str;
use crate::util::errors::add_err;
use crate::util::errors::wrap_io;
use crate::util::FedResult;
use filetime::{set_file_times, FileTime};

fn remove_file_times(path: &Path, verbose: bool) -> FedResult<()> {
    match set_file_times(path, FileTime::zero(), FileTime::zero()) {
        Ok(()) => Ok(()),
        Err(err) => return Err(add_err("failed to set file permissions while shredding", verbose, err)),
    }
}

