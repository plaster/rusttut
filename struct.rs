#[derive(Debug)]
struct User {
	login: String,
	email: String,
	internal_id: u32,
	active: bool,
	displayname: String,
}

/*
impl std::fmt::Debug for User {
	fn fmt( &self, f: &mut Formatter ) -> Result<(), Error> {
		...
	}
}
*/

/*
fn renamed(u: &User, newid: &str) {
	User {
		login: String::from(newid),
		internal_id: u.internal_id + 1024,
		..*u
	};
	/*
error[E0507]: cannot move out of borrowed content
  --> struct.rs:22:5
   |
22 | 		..*u
   | 		  ^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
  --> struct.rs:22:5
   |
22 | 		..*u
   | 		  ^^ cannot move out of borrowed content

error: aborting due to 2 previous errors
	*/
}
*/


fn main() {
	let user1 = User {
		login: String::from("user1"),
		email: String::from("user1@example.com"),
		internal_id: 42,
		active: true,
		displayname: String::from("User 1"),
	};
	// let user2 = renamed(&user1, "user2");
	/*
	let user2 = User {
		login: String::from("user2"),
		internal_id: user1.internal_id + 1024,
		..user1
	};

error[E0382]: use of partially moved value: `user1`
  --> struct.rs:43:19
   |
40 | 		..user1
   | 		  ----- value moved here
...
43 | 	println!("{:?}", user1);
   | 		                ^^^^^ value used here after move
   |
   = note: move occurs because `user1.email` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error
	*/

	println!("{:?}", user1);
}
