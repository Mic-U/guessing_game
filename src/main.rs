extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    println!("The Secret Number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to Read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
