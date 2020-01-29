pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }

    let mut j = 2;
    let mut primes: Vec<u32> = Vec::new();
    loop {
        if primes.len() as u32 == (n + 1) {
            return primes[n as usize];
        }
        if j_is_prime(j) {
            primes.push(j);
        }
        j += 1;
    }
    2
}
pub fn j_is_prime(j: u32) -> bool {
    if j == 0 {
        return false;
    }
    if j == 1 {
        return false;
    }
    if j == 2 {
        return true;
    }
    let mut i: u32 = j -1;
    while i > 1 {
        if j % i == 0 {
            return false;
        }
        i -= 1;
    }
    true
}
