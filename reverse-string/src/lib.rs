pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    // println!("{}", &input.reverse());
    // println!("{}", &input.rev());
    //input.to_string()

    /*for itr in input.chars() {
        itr.rev();

    }*/

    // println!("{}", input.chars().rev().collect::<String>());
    
    // input.to_string()
    // inspired by book --chapter 17 on strings.
    input.replace(input, &input.chars().rev().collect::<String>())
}
