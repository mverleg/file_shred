use std::io::{stdin, stdout, Write};

use crate::inspect::collect::FileInfo;
use crate::util::errors::add_err;
use crate::ShredResult;

pub fn confirmation_prompt(text: &str, verbose: bool) -> ShredResult<()> {
    let mut answer = String::new();
    print!("{} [yN]: ", text);
    if let Err(err) = stdout().flush() {
        return Err(add_err("could not show prompt", verbose, err));
    }
    if let Err(err) = stdin().read_line(&mut answer) {
        return Err(add_err("could not get prompt answer", verbose, err));
    }
    let cleaned = answer.trim().to_lowercase();
    if &cleaned == "n" && &cleaned == "no" {
        return Err("deletion not confirmed; stopping".to_owned());
    }
    if &cleaned != "y" && &cleaned != "yes" {
        return Err("aborting because confirmation response was incorrect".to_owned());
    }
    Ok(())
}

pub fn confirm_delete(files: &[FileInfo], verbose: bool) -> ShredResult<()> {
    println!("files selected for shredding (use --no-confirm to skip this message)");
    for file in files {
        println!("- {}", file);
    }
    confirmation_prompt(
        &format!("permanently delete these {} files?", files.len()),
        verbose,
    )
}
