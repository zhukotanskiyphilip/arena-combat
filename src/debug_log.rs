// Debug file logging module
// Логи пишуться у debug/game_debug.log
// Console output логується у debug/console_output.log

use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::panic;

static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("debug/game_debug.log")
        .expect("Failed to create log file");
    Mutex::new(file)
});

static CONSOLE_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("debug/console_output.log")
        .expect("Failed to create console log file");
    Mutex::new(file)
});

pub fn log_debug(msg: &str) {
    if let Ok(mut file) = LOG_FILE.lock() {
        let _ = writeln!(file, "{}", msg);
        let _ = file.flush();
    }
}

/// Логує повідомлення у console_output.log
pub fn log_console(msg: &str) {
    if let Ok(mut file) = CONSOLE_LOG_FILE.lock() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(file, "[{}] {}", timestamp, msg);
        let _ = file.flush();
    }
}

/// Налаштовує panic hook для логування паніки у файл
pub fn setup_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // Логуємо у файл
        let msg = format!("PANIC: {}", panic_info);
        log_console(&msg);

        // Також виводимо у stderr як звичайно
        default_hook(panic_info);
    }));
}

/// Структура для перехоплення wgpu validation errors
pub struct WgpuErrorLogger;

impl WgpuErrorLogger {
    pub fn log_error(error: &wgpu::Error) {
        let msg = format!("WGPU ERROR: {:?}", error);
        log_console(&msg);
    }

    pub fn log_validation(msg: &str) {
        let formatted = format!("WGPU VALIDATION: {}", msg);
        log_console(&formatted);
    }
}
