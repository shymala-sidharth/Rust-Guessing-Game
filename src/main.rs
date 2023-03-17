use std::io; //use of standard library std. Use the io library to obtain user input and print the result as output

fn main() {
    println!("Guess the number!");

    print!("Please input your guess.");

    let mut guess = String::new(); //introduce a mutable variable called 'guess' and binds String::new to it using = String::new is a function that returns a new instance of a string. String is a string type provided by the standard library.
    // created a mutable variable that is currently bound to a new, empty instance of a string.

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    print!("You Guessed: {guess}")

    
}
