use std::sync::atomic::{AtomicBool, Ordering};

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
