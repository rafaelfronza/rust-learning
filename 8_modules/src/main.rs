use std::io;
// using the rand crate
// use rand;
use rand::random;
use rand;
// use rand::rng;
// use rand::prelude::*;

fn standard_input() {
    let mut one_buffer = String::new();

    println!("Enter a message: ");
    io::stdin().read_line(&mut one_buffer);

    println!("buffer is {one_buffer}");
}

fn parse_string() {
    let mut two_buffer = String::new();

    println!("Enter a message: ");
    io::stdin().read_line(&mut two_buffer);

    println!("buffer is {two_buffer}");

    // let number = buffer.trim().parse::<i32>();
    let number: i32 = two_buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}

fn rust_crates() {
    // using the dependency for rand crate
    println!("\n=== crates ===");
    /*
        * Collection of rust source code files
        * Binary crates compile to produce an executable program
        * Library crates contain code for other programs to use
    */
    let number = random::<f64>();
    println!("number is {}", number);

    // let number = rng().gen_range(1..11);
    // println!("number is {}", number);
}

fn challenge() {
    /*
        * generates a random number between 1 and 100
        * user tries to guess the number
        * program tells if it's too high or low
        * repeats steps above (2 and 3) until guess
    */
    let random_number = rand::random_range(1..101);

    loop {
        let mut guess_number_str = String::new();

        println!("Enter a guess: ");
        io::stdin().read_line(&mut guess_number_str);

        let guess_number: u32 = guess_number_str.trim().parse().unwrap();

        if guess_number == random_number {
            println!("You guessed! The number is {}", random_number);
            break;
        } else {
            if guess_number > random_number {
                println!("...lower\n");
            } else if guess_number < random_number {
                println!("...higher\n");
            }
        }
    }
}

fn main() {
    // standard_input();
    // parse_string();
    // rust_crates();
    challenge();
}