extern crate rand;

use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	println!("Generated secret number: {}", secret_number);
    print!("input your guess: ");
	io::stdout().flush().ok().expect("failed to flush");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("failed to read line");
	println!("you guessed {}", guess);
}
