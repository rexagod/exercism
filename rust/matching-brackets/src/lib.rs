use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    println!("{}", string.to_owned());
    let allowed_chars = vec!['[', ']', '{', '}', '(', ')'];
    let mut stack: Vec<char> = Vec::new();
    let mut mapping = HashMap::new();
    mapping.insert(']', '[');
    mapping.insert(')', '(');
    mapping.insert('}', '{');
    for c in string.chars() {
        if allowed_chars.contains(&c) {
            if stack.len() == 0 {
                stack.push(c);
            } else {
                if mapping.contains_key(&c) {
                    if stack.last() == mapping.get(&c) {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                } else {
                    stack.push(c);
                }
            }
        }
        // for i in 0..brackets.len() {
        //     let bracket_type: &str = brackets[i];
        //     let open_bracket = bracket_type.chars().nth(0).unwrap();
        //     let close_bracket = bracket_type.chars().nth(1).unwrap();

        //     if c == open_bracket {
        //         counters[i] += 1;
        //     }
        //     if c == close_bracket {
        //         counters[i] -= 1;
        //         if counters[i] < 0 {
        //             possible &= false;
        //         }
        //     }
        // }
    }
    stack.len() == 0
}
