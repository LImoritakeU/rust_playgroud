use std::io::{self, Write};

use chrono::prelude::*;
use chrono::Utc;


pub fn calculate_retiration() {
    let mut age_str = "".to_string();
    let mut expect_retiration_age_str = "".to_string();

    print!("what is your current age? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age_str).expect("");
    let age: i32 = age_str.trim().parse().expect("");


    print!("at what age would you like to retire? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut expect_retiration_age_str).expect("");
    let retire_age: i32 = expect_retiration_age_str.trim().parse().expect("");

    println!("You have {} years left until you can retire.", retire_age - age);

    let utc = Utc::today();
    // 1
    let d = utc.with_year(utc.year() + retire_age - age).unwrap();

    println!("It's {}, so you can retire in {}", utc.year(), d.year());
}