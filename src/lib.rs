pub mod error;
pub mod expr;
pub mod parser;
pub mod scanner;
pub mod token;
use crate::{error::RloxError, expr::pretty_expr, parser::Parser, scanner::Scanner};
use std::fs;

pub fn run_file(path: &str) -> Result<(), RloxError> {
    let contents = fs::read_to_string(path)?;
    let had_error = run(contents)?;
    if had_error {
        std::process::exit(65);
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), RloxError> {
    use std::io::{self, Write};

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }

        if let Err(err) = run(line) {
            eprintln!("{err}");
        };
    }
    Ok(())
}

fn run(source: String) -> Result<bool, RloxError> {
    let tokens = Scanner::new(source).scan_tokens()?;
    let Some(expr) = Parser::new(tokens).parse() else {
        return Ok(false);
    };
    println!("{}", pretty_expr(&expr, 0));
    Ok(true)
}
