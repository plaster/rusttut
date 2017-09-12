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
}

fn main() {
	let r1 = Rectangle{ width: 10, height: 30 };
	println!("r1 = {:?}; area = {}", r1, r1.area());
	let r2 = r1.doubled();
	println!("r2 = {:?}; area = {}", r2, r2.area());
	let r3 = Rectangle{ width: 15, height: 20 };
	println!("r3 = {:?}; area = {}", r3, r3.area());
	println!("r1 can hold r3: {}", r1.can_hold(&r3));
	println!("r3 can hold r1: {}", r3.can_hold(&r1));
	println!("r2 can hold r3: {}", r2.can_hold(&r3));
	println!("r3 can hold r2: {}", r3.can_hold(&r2));
}
