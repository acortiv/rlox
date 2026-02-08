pub mod env;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
use crate::{error::RloxError, interpreter::Interpreter, parser::Parser, scanner::Scanner};
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

    let mut interpreter = Interpreter::default();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        let tokens = match Scanner::new(line.clone()).scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        let stmts = match Parser::new(tokens).parse() {
            Ok(stmts) => stmts,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        if let Err(err) = interpreter.interpret(stmts) {
            eprintln!("{err}");
        }
    }

    Ok(())
}

fn run(source: String) -> Result<bool, RloxError> {
    let tokens = Scanner::new(source).scan_tokens()?;

    let Ok(stmts) = Parser::new(tokens).parse() else {
        return Ok(false);
    };

    let mut interpreter = Interpreter::default();
    let Ok(_) = interpreter.interpret(stmts) else {
        return Ok(false);
    };

    Ok(true)
}
