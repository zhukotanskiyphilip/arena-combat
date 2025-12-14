// Debug file logging module
// Логи пишуться у debug/game_debug.log

use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("debug/game_debug.log")
        .expect("Failed to create log file");
    Mutex::new(file)
});

pub fn log_debug(msg: &str) {
    if let Ok(mut file) = LOG_FILE.lock() {
        let _ = writeln!(file, "{}", msg);
        let _ = file.flush();
    }
}
