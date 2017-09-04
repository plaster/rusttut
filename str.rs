fn main() {
	let mut a:String = "abc".to_string();
	println!("{}", a);
	a += "def";
	println!("{}", a);
	let b:&str = a.as_str();
	println!("{}", b);
	/*
	a += 1.0.to_string().as_str();
error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
  --> str.rs:8:2
   |
6  | 	let b:&str = a.as_str();
   | 	             - immutable borrow occurs here
7  | 	println!("{}", b);
8  | 	a += 1.0.to_string().as_str();
   | 	^ mutable borrow occurs here
9  | 	println!("{}", b);
10 | }
   | - immutable borrow ends here

error: aborting due to previous error
	println!("{}", b);
	*/
}
