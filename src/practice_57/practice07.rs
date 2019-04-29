use std::io::{self, Write, stdin};


pub fn rectangle() {
    let mut length_str = "".to_string();
    let mut width_str = "".to_string();
    const trans: f64 = 0.09290304;

    print!("What's the length of the room in feet? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut length_str).expect("Fail to read");
    let length: u32 = length_str.trim().parse().expect("Fail to parse int");


    print!("What's the length of the room in feet? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut width_str).expect("Fail to read");
    let width: u32 = width_str.trim().parse().expect("Fail to parse int");

    let square_feet = length * width;
    let s_meter = length as f64 * 0.3048 * width as f64 * 0.3048;
    let square_meter = square_feet as f64 * trans;


    // {:.3} round
    println!("You entered dimensions of {} feet by {} feet.\nThe area is\n{} square feet\n{:.3} square meters", length, width, square_feet, square_meter);
}