use std::io;


use std:: io;

fn main() {
    println!("Guess the number!");

    print!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    print!("You guessed: {guess}")

    
}
