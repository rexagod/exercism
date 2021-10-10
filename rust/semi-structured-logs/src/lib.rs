// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_fn = get_logger(level);
    log_fn(message)
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {} ", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {} ", message)
}
pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {} ", message)
}
// using match, way cleaner, compiler is smart to not ask for a 'default' here
pub fn get_logger(level: LogLevel) -> (fn(&str) -> String) {
    match level {
        LogLevel::Info => info,
        LogLevel::Warning => warn,
        LogLevel::Error => error,
        LogLevel::Debug => debug
    }
}