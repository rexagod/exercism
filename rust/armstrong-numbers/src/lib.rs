pub fn is_armstrong_number(num: u32) -> bool {
    sum_of_digits_to_power(num, count_digits(num)) == num
}

pub fn count_digits(num: u32) -> u32 {
    match num {
        0 => 0,
        _ => 1 + count_digits(num/10)
    }
}

pub fn sum_of_digits_to_power(num: u32, exp: u32) -> u32 {
    match num {
        0 => 0,
        _ => u32::pow(num%10, exp) + sum_of_digits_to_power(num/10, exp)
    }
}