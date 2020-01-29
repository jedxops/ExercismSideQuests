pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
/*    unimplemented!(
        "Sum the multiples of all of {:?} which are less than {}",
        factors,
        limit
    )*/
    let mut multiples: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    for i in 1..limit {
        for j in 0..factors.len() {
            if factors[j] != 0 && i % factors[j] == 0 && !multiples.contains(&i) {
                multiples.push(i);
                sum += i;
            }
        }
    }
    sum
}
