enum Move {
	TurnLeft,
	TurnRight,
	Forward,
	Sleep,
}

struct v2(usize, usize);

fn forward(p: &v2, v: &v2) -> v2 {
	v2( p.0 + v.0, p.1 + v.1)
}

fn turn_left(v: &v2) -> v2 {
	match v2 { ( x, y ) => v2( -y, x ) }
}
fn turn_right(v: &v2) -> v2 {
	match v2 { ( x, y ) => v2( y, -x ) }
}

/*
error[E0308]: mismatched types
  --> robot.rs:15:13
   |
15 | 	match v2 { ( x, y ) => v2( -y, x ) }
   | 	           ^^^^^^^^ expected fn item, found tuple
   |
   = note: expected type `fn(usize, usize) -> v2 {v2::{{constructor}}}`
              found type `(_, _)`

error[E0308]: mismatched types
  --> robot.rs:18:13
   |
18 | 	match v2 { ( x, y ) => v2( y, -x ) }
   | 	           ^^^^^^^^ expected fn item, found tuple
   |
   = note: expected type `fn(usize, usize) -> v2 {v2::{{constructor}}}`
              found type `(_, _)`

error: aborting due to 2 previous errors
*/

struct Robot {
	p: v2,
	v: v2,
}

fn next(r: &Robot, m: &Move) {
	match m {
		TurnLeft => Robot { v: turn_left(&r.v), ..*r },
		TurnRight => Robot { v: turn_right(&r.v), ..*r },
		Forward => Robot { p: forward(&r.p, &r.v), ..*r },
		Sleep => Robot { ..*r },
	};
}

fn main() {
}
