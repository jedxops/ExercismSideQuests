pub fn square(s: u32) -> u64 {
//    unimplemented!("grains of rice on square {}", s);
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s-1)
}

pub fn total() -> u64 {
//    unimplemented!();
    let mut sum: u64 = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum
}
