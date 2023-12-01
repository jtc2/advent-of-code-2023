use std::{io, process::exit};

fn guess() -> bool {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess = guess.trim().to_string();

    if guess == "" {
        exit(0);
    }

    let parsed_int = guess.parse::<i128>().expect("Invalid number");
    let secret_number = parsed_int + 1;

    if secret_number == parsed_int {
        println!("HOW?????");
    } else {
        println!("You lose");
    }

    println!("You guessed: {guess}. Very close to the secret number! Try again!");

    false
}

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    let mut result = guess();
    while !result {
        result = guess();
    }
}
