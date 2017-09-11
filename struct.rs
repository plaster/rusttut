struct User {
	login: String,
	email: String,
	internal_id: u32,
	active: bool,
	displayname: String,
}

fn main() {
	let user1 = User {
		login: String::from("user1"),
		email: String::from("user1@example.com"),
		internal_id: 42,
		active: true,
		displayname: String::from("User 1"),
	};
}
