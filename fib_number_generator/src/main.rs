// Fibonacci Number Generator
use std::io;

fn main() {
    println!("Please input the Fibonacci number you would like to generate");

    let mut fib_digit = String::new();

    io::stdin()
        .read_line(&mut fib_digit)
        .expect("Could not read line");

    let fib_digit: usize = fib_digit
        .trim()
        .parse()
        .expect("That's not a positive integer!");

    println!("Fib digit to be processed: {}", fib_digit);

    let fib_number: usize = fib(fib_digit);

    println!("The Fibonacci number is: {}", fib_number);
}

fn fib(num: usize) -> usize {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return fib(num - 1) + fib(num - 2)
    }
}
