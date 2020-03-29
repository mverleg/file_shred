use std::io::{stdin, stdout, Write};

use crate::ShredResult;
use crate::inspect::collect::FileInfo;

pub fn confirmation_prompt(text: &str) -> ShredResult<()> {
    let mut answer = String::new();
    print!("{} [yN]: ", text);
    if let Err(err) = stdout().flush() {
        unimplemented!()  //TODO @mark:
    }
    if let Err(err) = stdin().read_line(&mut answer) {
        unimplemented!()  //TODO @mark:
    }
    let cleaned = answer.trim().to_lowercase().to_string();
    if &cleaned == "n" && &cleaned == "no" {
        return Err("deletion not confirmed; stopping".to_owned())
    }
    if &cleaned != "y" && &cleaned != "yes" {
        return Err("aborting because confirmation response was incorrect".to_owned())
    }
    Ok(())
}

pub fn confirm_delete(files: &[FileInfo]) -> ShredResult<()> {
    //TODO @mark: --no-confirm
    println!("files selected for shredding (use --no-confirm to skip this message)");
    for file in files {
        println!("- {}", file);
    }
    confirmation_prompt(&format!("permanently delete these {} files?", files.len()))
}