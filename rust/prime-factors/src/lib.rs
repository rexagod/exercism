pub fn factors(mut n: u64) -> Vec<u64> {
    let sq: u64 = (n as f64).sqrt() as u64;
    let mut prime_factors = Vec::new();
    for i in 2..=sq {
        while n%i == 0 {
            prime_factors.push(i);
            n /= i;
        }
    }
    if n > 1 {
        prime_factors.push(n);
    }
    prime_factors
}
