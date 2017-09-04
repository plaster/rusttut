fn matchx(n:isize) {
	match n {
		0 => println!("zero"),
		// https://twitter.com/keitarogu/status/884353060188725249
		1...10 => println!("small"),
		/*
		1..10 => println!("small"),
error: exclusive range pattern syntax is experimental (see issue #37854)
 --> match.rs:4:3
  |
4 | 		1..10 => println!("small"),
  | 		^^^^^

error: aborting due to previous error
		*/
		x => println!("{} is large", x),
	}
}

fn main() {
	/*
	let a : [isize;10] = 1..10;
	println!("{:?}", a);
error[E0308]: mismatched types
  --> match.rs:21:23
   |
21 | 	let a : [isize;10] = 1..10;
   | 	                     -----
   | 	                     |
   | 	                     expected array of 10 elements, found struct `std::ops::Range`
   | 	                     in this macro invocation
   |
   = note: expected type `[isize; 10]`
              found type `std::ops::Range<{integer}>`

error: aborting due to previous error
	*/


	/*
	println!("{:?}", 1...10);
error: inclusive range syntax is experimental (see issue #28237)
  --> match.rs:22:19
   |
22 | 	println!("{:?}", 1...10);
   | 	                 ^^^^^^

error: aborting due to previous error
	*/
	matchx(3);
	matchx(0);
	matchx(10);
	matchx(30);
}
