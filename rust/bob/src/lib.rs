pub fn reply(message: &str) -> &str {
    println!("{}", message);
    match (is_question(message.trim()), is_yelling(message.trim()), is_silent(message.trim())) {
        (_, _, true) => "Fine. Be that way!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (_, _, _) => "Whatever."
    }
}

pub fn is_question(message: &str) -> bool {
    return message.ends_with('?')
}

pub fn is_yelling(message: &str) -> bool {
    message.to_owned().chars()
        .any(|c| c >= 'A' && c <= 'Z') &&       // contains atleast one uppercase character
        message.to_owned().chars()
            .all(|c| !(c >= 'a' && c <= 'z'))   // does not contain any lowercase characters
}

pub fn is_silent(message: &str) -> bool {
    message == ""
}