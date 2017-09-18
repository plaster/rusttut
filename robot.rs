enum Move {
	TurnLeft,
	TurnRight,
	Forward,
	Sleep,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct Robot {
	p: v2,
	v: v2,
}

impl Robot {
	fn iamhere(&self) {
		println!("({}, {})", self.p.0, self.p.1);
	}
	fn next(&self, m: Move) -> Robot {
		match m {
			Move::TurnLeft => Robot { v: turn_left(&self.v), ..self.clone()},
			Move::TurnRight => Robot { v: turn_right(&self.v), ..self.clone()},
			Move::Forward => Robot { p: forward(&self.p, &self.v), ..self.clone()},
			Move::Sleep => Robot { ..self.clone()},
		}
	}
}

fn main() {
	let r = Robot { p: v2(0, 0), v: v2(1, 0) };
	r.iamhere();
	let r2 = r.next(Move::Forward);
	r2.iamhere();
	r.iamhere();
}
