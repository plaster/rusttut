fn rebind() {
	let sum = 0;
	for i in 0..10 {
		let sum = sum + i;
	}
	println!("{}", sum);
}
fn reassignment() {
	let mut sum = 0;
	for i in 0..10 {
		sum = sum + i;
	}
	println!("{}", sum);
}

fn main() {
	rebind();
	reassignment();
}
