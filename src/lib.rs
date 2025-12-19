pub mod error;
pub mod expr;
pub mod scanner;
pub mod token;

use crate::error::{HAD_ERROR, RloxError};
use crate::scanner::Scanner;
use std::fs;
use std::sync::atomic::Ordering;

pub fn run_file(path: &str) -> Result<(), RloxError> {
    let contents = fs::read_to_string(path)?;
    run(contents)?;
    if HAD_ERROR.load(Ordering::Relaxed) {
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
        run(line)?;
        HAD_ERROR.store(false, Ordering::Relaxed);
    }
    Ok(())
}

fn run(source: String) -> Result<(), RloxError> {
    let tokens = Scanner::new(source).scan_tokens()?;
    for token in tokens {
        println!("Current token: {:?}", token);
    }
    Ok(())
}
