pub fn factors(n: u64) -> Vec<u64> {
//    unimplemented!("This should calculate the prime factors of {}", n)
    let mut v: Vec<u64> = Vec::new();
    let mut m = n;
    for i in 2..=m {
        // let tup = i_is_factor(n, i);
        if m % i == 0 && i_is_prime(i) {
            loop {
                if m % i != 0 {
                    break;
                }
                m = m / i;
                v.push(i);
                if m == 1 {
                    return v;
                }
            }
        }
    }
    v
}


pub fn _i_is_factor(n: u64, mut i: u64) -> (bool, u64) {
    if i == 0 || i == 1 || i < 0 || i > n {
        return (false, 0);
    }
    let mut j: u64 = 0;
    loop {
        if i == n {
            return (true, j + 1);
        }
        if i > n {
            return (false, 0);
        }
        i = i * i;
        j += 1;
    }
    (false, 0)
}

// copied this function from my `nth` prime solution
// it is my own function that I wrote so I can use it
pub fn i_is_prime(i: u64) -> bool {
    if i == 0 {
        return false;
    }
    if i == 1 {
        return false;
    }
    if i == 2 {
        return true;
    }
    let mut j: u64 = i -1;
    while j > 1 {
        if i % j == 0 {
            return false;
        }
        j -= 1;
    }
    true
}
