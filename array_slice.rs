fn main() {
	let a: [isize; 3] = [1, 2, 3];
	let b: &[isize] = &a;
	// println!("{}", a);
	println!("{:?}", a);
	println!("{:?}", &a);
	println!("{:?}", b);
	// println!("{}", b);
}
