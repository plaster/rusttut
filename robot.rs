enum Move {
	TurnLeft,
	TurnRight,
	Forward,
	Sleep,
}

struct v2(isize, isize);

fn forward(p: &v2, v: &v2) -> v2 {
	v2( p.0 + v.0, p.1 + v.1)
}

fn turn_left(v: &v2) -> v2 {
	match *v { v2( x, y ) => v2( -y, x ) }
}
fn turn_right(v: &v2) -> v2 {
	match *v { v2( x, y ) => v2( y, -x ) }
}

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
	/*
error[E0507]: cannot move out of borrowed content
  --> robot.rs:28:45
   |
28 | 		TurnLeft => Robot { v: turn_left(&r.v), ..*r },
   | 		                                          ^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
  --> robot.rs:29:47
   |
29 | 		TurnRight => Robot { v: turn_right(&r.v), ..*r },
   | 		                                            ^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
  --> robot.rs:30:48
   |
30 | 		Forward => Robot { p: forward(&r.p, &r.v), ..*r },
   | 		                                             ^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
  --> robot.rs:31:22
   |
31 | 		Sleep => Robot { ..*r },
   | 		                   ^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
  --> robot.rs:31:22
   |
31 | 		Sleep => Robot { ..*r },
   | 		                   ^^ cannot move out of borrowed content

error: aborting due to 5 previous errors
	*/
}

fn main() {
}
