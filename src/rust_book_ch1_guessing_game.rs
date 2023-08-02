use std::io;

pub fn main_guessing_game() {
    println!("Guessing Game!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)           // pass mutable reference
        .expect("Failed to read line");  // crash or return number of bytes read

    println!("You guessed: {guess}");
}
