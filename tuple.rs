fn main() {
	let a: (isize, f64, &str) = ( 10, 3.14, "abcdefg");
	println!("{} {} {}", a.1, a.0, a.2 );
	/*
	let x = 0;
	println!("{} {} {}", a.1, a.x, a.2 );
error[E0609]: no field `x` on type `(isize, f64, &str)`
 --> tuple.rs:5:30
  |
5 | 	println!("{} {} {}", a.1, a.x, a.2 );
  | 	                            ^

error: aborting due to previous error
	*/
}
