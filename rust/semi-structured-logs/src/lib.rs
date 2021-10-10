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
    return "[INFO]: ".to_owned() + message
}
pub fn warn(message: &str) -> String {
    return "[WARNING]: ".to_owned() + message
}
pub fn error(message: &str) -> String {
    return "[ERROR]: ".to_owned() + message
}
pub fn debug(message: &str) -> String {
    return "[DEBUG]: ".to_owned() + message
}
// tried to return a function, compiler requires me to have an else
pub fn get_logger(level: LogLevel) -> fn(&str) -> String {
    if level == LogLevel::Info {
        return info
    } else if level == LogLevel::Warning {
        return warn
    } else if level == LogLevel::Error {
        return error
    } else {
        return debug
    }
}