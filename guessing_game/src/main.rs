extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numbo?");

    let secret_numbo = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_numbo);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to re-qad line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You gussed: {}", guess);

        match guess.cmp(&secret_numbo) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too wumbo!"),
            Ordering::Equal => {
                println!("Who touch my spaget");
                break;
            }
        }
    }
}
