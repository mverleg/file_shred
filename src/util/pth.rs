use ::std::path::PathBuf;
use std::path::Path;

pub fn determine_output_path(input_path: &Path, extension: &str, output_dir: Option<&Path>) -> PathBuf {
    let name = {
        let mut n = input_path.file_name().unwrap().to_os_string();
        n.push(extension);
        n
    };
    match output_dir {
        Some(p) => {
            let mut p = p.to_owned();
            p.push(name);
            p
        },
        None => {
            let mut p = input_path.to_owned();
            p.pop();
            p.set_file_name(name);
            p
        }
    }
}
