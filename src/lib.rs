mod error;

use crate::error::HAD_ERROR;
use std::sync::atomic::Ordering;
use std::{fs, io};

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
