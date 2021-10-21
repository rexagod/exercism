/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.trim();
    if stripped_code.len() <= 1 {
        return false
    }
    let mut valid_index = 0;
    let mut sum: i32 = 0;
    for c in stripped_code.chars().rev() {
        if c >= '0' && c <= '9' {
            valid_index += 1;
            let mut num_c = c as u8 - '0' as u8;
            if valid_index%2 == 0 {
                num_c = match num_c*2/10 {
                    0 => num_c*2,
                    _ => num_c*2 - 9
                }
            }
            sum += num_c as i32;
        } else if c != ' ' {
            return false
        }
    }
    return sum % 10 == 0
}
