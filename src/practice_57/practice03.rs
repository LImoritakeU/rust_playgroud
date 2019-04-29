use std::io::{self, Write};
/*
many ways to concat sentence.
*/

pub fn use_quote() {
    let mut quote = String::new();
    let mut person = String::new();

    print!("What is the quote?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut quote).expect(" ");

    print!("Who said it?");
    io::stdout().flush().unwrap();
    let quote = quote.trim_end();
    io::stdin().read_line(&mut person).expect(" ");
    let person = person.trim_end();


    // 1
    println!("{} says, \"{}\"", person, quote);


    // 2
    let mut sentence = &[person, " says, \"", quote, "\""].concat();
    println!("{}", sentence);
}