use std::io::{self, Write};

pub fn pizza_group() {
    let mut s_people = "".to_string();
    let mut s_piece = "".to_string();
    const pieces_per_pizza: i32 = 8;

    print!("How many people? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s_people).expect("Fail to read");
    let people: i32 = s_people.trim().parse().expect("Please input integer");

    print!("How many pieces? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s_piece).expect("Fail to read");
    let piece: i32 = s_piece.trim().parse().expect("Please input integer");

    let total = piece * pieces_per_pizza;


    println!("{} people with {} pizzas", people, piece);
    println!("each people get {} pieces of pizza.", total / people);
    println!("There are {} leftover pieces", total % people);
}