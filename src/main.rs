mod generate_ast;
mod scanner;
use clap::Parser;
use scanner::{Scanner, Token};
use std::any::Any;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

#[allow(dead_code)]
fn main() -> std::io::Result<()> {
    generate_ast! {
        Expr {
            Binary(left: Expr, operator: Token, right: Expr),
            Grouping(expression: Expr),
            Literal(value: dyn Any),
            Unary(operator: Token, right: Expr)
        }
    }

    define_visitor! {
        Expr, Binary, Literal, Grouping, Unary
    }

    let args = Args::parse();
    if let Some(file) = args.file.as_deref() {
        println!("run from file");
        let contents = fs::read_to_string(file)?;
        let mut my_scanner = Scanner::new(contents);
        my_scanner.scan_tokens();
    } else {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("grr");

        let source = input.trim();
        let mut my_scanner = Scanner::new(source.to_string());
        my_scanner.scan_tokens();
    }

    Ok(())
}
