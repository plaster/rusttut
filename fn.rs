fn add(x:isize, y:isize) -> isize {
	(x + y) % 42
}

fn main() {
	let f: fn(isize, isize) -> isize = add;
	for i in 0 .. 30 {
		for j in 0 .. 30 {
			let s = f( i, j );
			let p:&str = if s < 10 { " " } else { "" };
			print!("| {}{}", p, s);
		}
		println!(" |");
	}
}
