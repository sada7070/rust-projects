use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    let mut number_of_guesses: u32 = 0;

    loop {
        println!("Please input your guess:");

        number_of_guesses += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOU WIN!!");
                println!("You took {} attempts to guess the number.", number_of_guesses);
                break;
            }
        }
    }
}
