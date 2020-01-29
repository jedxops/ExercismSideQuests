pub fn square_of_sum(n: u32) -> u32 {
//    unimplemented!("square of sum of 1...{}", n)
    let mut sum: u32 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
//    unimplemented!("sum of squares of 1...{}", n)
    let mut sum: u32 = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: u32) -> u32 {
/*    unimplemented!(
        "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
        n = n,
    )*/
    square_of_sum(n) - sum_of_squares(n)
}
