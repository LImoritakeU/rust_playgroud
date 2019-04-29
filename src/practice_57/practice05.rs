use std::io::{self, Write};

pub fn calculate() {
    let mut num1: i32;
    let mut num2: i32;
    let mut s1 = "".to_string();
    let mut s2 = "".to_string();


    print!("What's the first number? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s1).expect("Failed to read");
    num1 = s1.trim().parse().expect("Please input a number");

    print!("What's the second number? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s2).expect("Failed to read");
    num2 = s2.trim().parse().expect("Please input a number");

    println!("{} + {} = {}", num1, num2, num1 + num2);
}