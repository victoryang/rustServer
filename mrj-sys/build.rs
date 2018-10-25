use std::process::Command;

fn main() {
	Command::new("make").args(&["-C", "mrj/"])
						.status().unwrap();
}