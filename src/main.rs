use rlox::{error::RloxError, run_file, run_prompt};
use std::env;

fn main() -> Result<(), RloxError> {
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
