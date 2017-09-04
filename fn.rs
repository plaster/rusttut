fn add(x:isize, y:isize) -> isize {
	(x + y) % 42
}

fn mul(x:isize, y:isize) -> isize {
	(x * y) % 42
}

fn main() {
	let fs: [ fn(isize, isize) -> isize; 2 ] = [ add, mul ];
	for i in 0 .. 20 {
		for j in 10 .. 30 {
			for f in &fs {
				let s = f( i, j );
				let p:&str = if s < 10 { " " } else { "" };
				print!("| {}{}", p, s);
			}
		}
		println!(" |");
	}
}
