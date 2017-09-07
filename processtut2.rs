use std::process::{Command, Stdio};
use std::io::{Write, self};

fn main() {
	let mut child = Command::new("/bin/cat")
		.arg("-n")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute");
	{
		let child_in = child.stdin.as_mut().expect("failed to get stdin");
		child_in.write_all(b"test").expect("failed to write");
	}

	let child_output = child
		.wait_with_output()
		.expect("faild to wait on child");


	let stdout = io::stdout();
	let mut handle = stdout.lock();
	handle.write(child_output.stdout.as_slice()).expect("failed to write");
}
