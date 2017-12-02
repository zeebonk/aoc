use std::io::prelude::*;
use std::fs::File;

fn main() {
	let mut f = File::open("input/day_1.txt").unwrap();
	let mut buffer = String::new();
	let _ = f.read_to_string(&mut buffer);

	let count = buffer.chars().fold(
		0,
		|sum ,c| sum + match c {
			'(' => 1,
			')' => -1,
			_ => 0,
		}
	);

	println!("{}", count);
}
