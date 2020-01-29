pub fn raindrops(n: u32) -> String {
//    unimplemented!("what sound does Raindrop #{} make?", n)
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }
    let mut string = String::new();
    if n % 3 == 0 {
        string.push_str("Pling");
    }
    if n % 5 == 0 {
        string.push_str("Plang");
    }
    if n % 7 == 0 {
        string.push_str("Plong");
    }
    string
}
