mod scanner;
use clap::Parser;
use scanner::Scanner;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if let Some(file) = args.file.as_deref() {
        println!("run from file");
        let contents = fs::read_to_string(file)?;
        let mut my_scanner = Scanner::new(contents);
        my_scanner.scan_tokens();
    } else {
        println!("run line by line");
    }

    Ok(())
}
