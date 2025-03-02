use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(_file) = args.file.as_deref() {
        println!("run from file");
    } else {
        println!("run line by line");
    }
}
