use std::collections::HashSet;


pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    for &factor in factors {
        if factor != 0 {
            let mut multiplier = 1;
            while multiplier * factor < limit {
                set.insert(multiplier * factor);
                multiplier += 1;
            }
        }
    }
    set.iter().sum()
}
