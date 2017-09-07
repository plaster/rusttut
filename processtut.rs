use std::process::Command;

fn main() {
	let mut child = Command::new("/bin/ls")
		.arg("-l")
		.spawn()
		.expect("failed to execute child");

	let ecode = child.wait().expect("failed to wait");

	assert!(ecode.success());
}
