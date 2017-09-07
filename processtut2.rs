use std::process::{Command, Stdio};
use std::io::{Write, self};

fn main() {
	let child1 = Command::new("/bin/ls")
		.arg("-l")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute");
	let child2 = Command::new("/bin/cat")
		.arg("-n")
		.stdin(std::process::Stdio::from(child1.stdout.expect("failed to get stdout")))
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute");

	let child_output = child2
		.wait_with_output()
		.expect("faild to wait on child2");


	let stdout = io::stdout();
	let mut handle = stdout.lock();
	handle.write(child_output.stdout.as_slice()).expect("failed to write");
}
