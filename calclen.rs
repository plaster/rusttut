fn calclen(s: String) -> usize {
	s.len()
}

fn main() {
	let s = String::from("あー");
	println!("{}", calclen(s));
	/*
	println!("{}", s);
error[E0382]: use of moved value: `s`
 --> calclen.rs:8:17
  |
7 | 	println!("{}", calclen(s));
  | 	                       - value moved here
8 | 	println!("{}", s);
  | 	               ^ value used here after move
  |
  = note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error
	*/
}
