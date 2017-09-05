use std::io::{self, Write};

fn main() {
    println!("Guess the number!");
    print!("input your guess: ");
	io::stdout().flush().ok().expect("failed to flush");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("failed to read line");
	println!("you guessed {}", guess);
}
