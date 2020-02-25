use ::std::path::Path;
use ::std::path::PathBuf;

pub fn determine_output_path(
    input_path: &Path,
    extension: &str,
    output_dir: Option<&Path>,
) -> PathBuf {
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
        }
        None => {
            let mut p = input_path.to_owned();
            p.set_file_name(name);
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_absolute_no_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("/alpha/beta/gamma.txt"),
            ".enc",
            None
        );
        assert_eq!(out_pth, PathBuf::from("/alpha/beta/gamma.txt.enc"));
    }

    #[test]
    fn output_relative_no_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("alpha/beta/gamma.txt"),
            ".enc",
            None
        );
        assert_eq!(out_pth, PathBuf::from("alpha/beta/gamma.txt.enc"));
    }

    #[test]
    fn output_just_name_no_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("name.txt"),
            ".enc",
            None
        );
        assert_eq!(out_pth, PathBuf::from("name.txt.enc"));
    }

    #[test]
    fn output_absolute_with_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("/alpha/beta/gamma.txt"),
            ".enc",
            Some(&PathBuf::from("/output/enc"))
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt.enc"));
    }

    #[test]
    fn output_relative_with_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("alpha/beta/gamma.txt"),
            ".enc",
            Some(&PathBuf::from("/output/enc"))
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt.enc"));
    }

    #[test]
    fn output_just_name_with_output_dir() {
        let out_pth = determine_output_path(
            &PathBuf::from("name.txt"),
            ".enc",
            Some(&PathBuf::from("/output/enc"))
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/name.txt.enc"));
    }
}
