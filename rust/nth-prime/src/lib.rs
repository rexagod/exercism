pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        _ => {
            let mut index = 0;
            let mut num = 1;
            while index != n {
                num += 2;
                if is_prime(num) {
                    index += 1;
                }
            }
            num
        }
    }
}

pub fn is_prime(n: u32) -> bool {
    let mut i = 3;
    while i as f64 <= (n as f64).sqrt() {
        if n%i == 0 {
            return false
        }
        i += 2;
    }
    true
}