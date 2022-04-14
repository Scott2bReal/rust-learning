use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess_counter = 0;

    const GUESS_LIMIT: u32 = 10;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guess_counter += 1;

        match guess_counter.cmp(&GUESS_LIMIT) {
            Ordering::Less => println!("You have {} guesses left!", (GUESS_LIMIT - guess_counter)),
            Ordering::Equal => {
                println!("You are out of guesses!");
                println!("The secret number was: {}", secret_number);
                break;
            }
            Ordering::Greater => println!("This will never happen!"),
        }
    }
}
