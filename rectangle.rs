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
}

fn main() {
	let r = Rectangle{ width: 10, height: 30 };
	println!("r = {:?}; area = {}", r, r.area());
	let r2 = r.doubled();
	println!("r2 = {:?}; area = {}", r2, r2.area());
	println!("r = {:?}; area = {}", r, r.area());
}
