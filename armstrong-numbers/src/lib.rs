pub fn is_armstrong_number(num: u32) -> bool {
//    unimplemented!("true if {} is an armstrong number", num)
    let mut num2 = num;
    let digits = digits(num2);
    let mut digit_vec: Vec<u32> = Vec::new();
    let mut i: u32 = 1;
    while i <= digits {
        digit_vec.push(num2 % 10);
        num2 /= 10; 
        i += 1;
    }
    let mut sum: u32 = 0;
    for i in 0..digit_vec.len() {
        sum += digit_vec[i].pow(digits);
    }
    if sum != num {
        return false;
    }
    true
}

pub fn digits(num: u32) -> u32 {
    let mut i:u32 = 1;
    loop {
        if num < 10u32.pow(i) {
            return i;
        }
        i += 1;
    }
    0
}
