use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn main_guessing_game() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=101);    // range

    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)           // pass mutable reference
        .expect("Failed to read line");  // crash or return number of bytes read

    let guess: u32 = guess                    // mark as u32
        .trim()                               // remove newline
        .parse()                              // parse to u32
        .expect("Please type a number!");     // crash or return number

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
