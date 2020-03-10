use ::std::path::Path;
use ::std::path::PathBuf;

use crate::config::typ::Extension;

pub fn determine_output_path(
    input_path: &Path,
    extension: Extension,
    output_dir: Option<&Path>,
) -> PathBuf {
    let name = match extension {
        Extension::Add(ext) => {
            let original_name = input_path.file_name().unwrap().to_os_string();
            let mut new_name = original_name;
            new_name.push(ext);
            new_name
        },
        Extension::Strip => {
            let mut new_name = input_path.file_stem().unwrap().to_os_string();
            let original_name = input_path.file_name().unwrap().to_os_string();
            if original_name == new_name {
                eprintln!("warning: encrypted file {} has no extension, so `~` will be appended \
                to make output name differ from input", original_name.to_string_lossy());
                new_name.push("~");
            }
            new_name
        },
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

    //TODO @mark: tests for stripping extension
    //TODO @mark: tests for stripping if no extension

    #[test]
    fn output_absolute_no_output_dir_add_ext() {
        let out_pth = determine_output_path(&PathBuf::from("/alpha/beta/gamma.txt"), Extension::Add(".enc"), None);
        assert_eq!(out_pth, PathBuf::from("/alpha/beta/gamma.txt.enc"));
    }

    #[test]
    fn output_absolute_no_output_dir_strip_ext() {
        let out_pth = determine_output_path(&PathBuf::from("/alpha/beta/gamma.txt.enc"), Extension::Strip, None);
        assert_eq!(out_pth, PathBuf::from("/alpha/beta/gamma.txt"));
    }

    #[test]
    fn output_relative_no_output_dir_add_ext() {
        let out_pth = determine_output_path(&PathBuf::from("alpha/beta/gamma.txt"), Extension::Add(".enc"), None);
        assert_eq!(out_pth, PathBuf::from("alpha/beta/gamma.txt.enc"));
    }

    #[test]
    fn output_relative_no_output_dir_strip_ext() {
        let out_pth = determine_output_path(&PathBuf::from("alpha/beta/gamma.txt.enc"), Extension::Strip, None);
        assert_eq!(out_pth, PathBuf::from("alpha/beta/gamma.txt"));
    }

    #[test]
    fn output_just_name_no_output_dir_add_ext() {
        let out_pth = determine_output_path(&PathBuf::from("name.txt"), Extension::Add(".enc"), None);
        assert_eq!(out_pth, PathBuf::from("name.txt.enc"));
    }

    #[test]
    fn output_just_name_no_output_dir_strip_ext() {
        let out_pth = determine_output_path(&PathBuf::from("name.txt.enc"), Extension::Strip, None);
        assert_eq!(out_pth, PathBuf::from("name.txt"));
    }

    #[test]
    fn output_absolute_with_output_dir_add_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("/alpha/beta/gamma.txt"),
            Extension::Add(".enc"),
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt.enc"));
    }

    #[test]
    fn output_absolute_with_output_dir_strip_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("/alpha/beta/gamma.txt.enc"),
            Extension::Strip,
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt"));
    }

    #[test]
    fn output_relative_with_output_dir_add_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("alpha/beta/gamma.txt"),
            Extension::Add(".enc"),
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt.enc"));
    }

    #[test]
    fn output_relative_with_output_dir_strip_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("alpha/beta/gamma.txt.enc"),
            Extension::Strip,
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/gamma.txt"));
    }

    #[test]
    fn output_just_name_with_output_dir_add_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("name.txt"),
            Extension::Add(".enc"),
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/name.txt.enc"));
    }

    #[test]
    fn output_just_name_with_output_dir_strip_ext() {
        let out_pth = determine_output_path(
            &PathBuf::from("name.txt.enc"),
            Extension::Strip,
            Some(&PathBuf::from("/output/enc")),
        );
        assert_eq!(out_pth, PathBuf::from("/output/enc/name.txt"));
    }

    #[test]
    fn strip_without_extension() {
        let out_pth = determine_output_path(
            &PathBuf::from("name"),
            Extension::Strip,
            None,
        );
        assert_eq!(out_pth, PathBuf::from("name~"));
    }
}
