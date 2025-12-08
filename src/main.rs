mod error;
mod token;

use crate::error::{HAD_ERROR, error};
use std::sync::atomic::Ordering;
use std::{env, fs, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;
    run(&contents);
    if HAD_ERROR.load(Ordering::Relaxed) {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    use std::io::{self, Write};

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        run(&line);
        HAD_ERROR.store(false, Ordering::Relaxed);
    }
    Ok(())
}

fn run(source: &str) {
    let tokens = scan_tokens(&source);
    for token in tokens {
        println!("Current token: {}", token);
    }
}
