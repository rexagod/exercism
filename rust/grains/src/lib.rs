pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    1 << s - 1
}

pub fn total() -> u64 {
    let mut ans = 0;
    for i in 1..=64 {
        ans += square(i);
    }
    ans
}
