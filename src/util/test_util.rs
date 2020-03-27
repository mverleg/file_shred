use ::std::{env, fs};

use ::rand::Rng;
use ::rand::thread_rng;

use crate::util::base64::u64_to_base64str;
use std::path::PathBuf;

pub fn make_test_dir() -> PathBuf {
    let mut dir = env::temp_dir();
    let name = format!("file_endec_test_{}",
        u64_to_base64str(thread_rng().gen()));
    dir.push(name);
    fs::create_dir(&dir).unwrap();
    dir
}
