#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn doubled(&self) -> Rectangle {
		Rectangle {
			width: self.width * 2,
			height: self.height * 2,
		}
	}
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
	fn square(size: u32) -> Rectangle {
		Rectangle{ width: size, height: size }
	}
}

fn main() {
	let r1 = Rectangle{ width: 10, height: 30 };
	println!("r1 = {:?}; area = {}", r1, r1.area());
	let r2 = r1.doubled();
	println!("r2 = {:?}; area = {}", r2, r2.area());
	let r3 = Rectangle{ width: 15, height: 20 };
	println!("r3 = {:?}; area = {}", r3, r3.area());
	let r4 = Rectangle::square(80);
	println!("r4 = {:?}; area = {}", r4, r4.area());

	println!("r1 can hold r2: {}", r1.can_hold(&r2));

	println!("r2 can hold r1: {}", r2.can_hold(&r1));

	println!("r1 can hold r3: {}", r1.can_hold(&r3));
	println!("r2 can hold r3: {}", r2.can_hold(&r3));

	println!("r3 can hold r1: {}", r3.can_hold(&r1));
	println!("r3 can hold r2: {}", r3.can_hold(&r2));

	println!("r1 can hold r4: {}", r1.can_hold(&r4));
	println!("r2 can hold r4: {}", r2.can_hold(&r4));
	println!("r3 can hold r4: {}", r3.can_hold(&r4));

	println!("r4 can hold r1: {}", r4.can_hold(&r1));
	println!("r4 can hold r2: {}", r4.can_hold(&r2));
	println!("r4 can hold r3: {}", r4.can_hold(&r3));
}
