pub fn verse(n: i32) -> String {
//    unimplemented!("emit verse {}", n)
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    if n == 2 {
        return "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string();
    }
    else if n > 2 {
        let mut string = String::new();
        string.push_str(&n.to_string());
        string.push_str(" bottles of beer on the wall, ");
        string.push_str(&n.to_string());
        string.push_str(" bottles of beer.\nTake one down and pass it around, ");
        string.push_str(&(n-1).to_string());
        string.push_str(" bottles of beer on the wall.\n");
        return string;
    }
    "ddd".to_string()
}

pub fn sing(start: i32, end: i32) -> String {
    let mut string = String::new();
    let mut i: i32 = start;
    while i >= end {
        &string.push_str(&verse(i).to_string());
        if i != end {
            &string.push_str("\n");
        }
        i -= 1;;
    }
    string
}
