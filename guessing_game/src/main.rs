// https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html

extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	// println!("Generated secret number: {}", secret_number);

	loop {
		print!("input your guess: ");
		io::stdout().flush().ok().expect("failed to flush");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("failed to read line");
		// println!("you guessed {}", guess);
		match guess.as_str() {
			"" => {
				println!("OK, the answer is {}. bye...", secret_number);
				break;
			},
			_ => (),
		};

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(msg) => {
				println!("invalid input: {}", msg);
				continue;
			},
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("Just!");
				break;
			},
		};
	}
}
