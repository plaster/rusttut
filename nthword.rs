fn nth_word(s: &str, n: usize) -> &str {
	let bytes = s.as_bytes();
	for ( i, &item ) in bytes.iter().enumerate() {
		if item == b' ' {
			if n == 0 {
				return &s[ 0 .. i ];
			} else {
				return nth_word(&s[ i+1 .. s.len() ], n-1)
			}
		}
	}
	return s;
}

fn main() {
	let s = "quick brown fox jumps over the lazy dog.";
	for n in 0 .. 10 {
		println!("{}: {}", n, nth_word(&s, n));
	}
	/*
0: quick
1: brown
2: fox
3: jumps
4: over
5: the
6: lazy
7: dog.
8: dog.
9: dog.
	*/
}
