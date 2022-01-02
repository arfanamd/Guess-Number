#![warn(deprecated_in_future)]
#![forbid(deprecated)]
#![allow(non_camel_case_types, non_snake_case)]

mod guess;
use guess::guess::deck;
use std::io;

const MARK: &str = "\x1b[s";
const JUMP: &str = "\x1b[u";
const WIPE: &str = "\x1b[15M";

fn main() {
	print!("{}", MARK);

	let mut input: i32   = 0;
	let mut count: usize = 0;
	let mut cards = deck::generate();

	while count < 5 {
		println!("{}", cards);
		print!("----------\n:> ");

		let mut buff  = String::new();
		io::stdin()
			.read_line(&mut buff)
			.expect("Fail to read line");

		input = buff.trim().parse::<i32>().unwrap();

		if input > 3 || input < 1 {
			print!("{}", JUMP);
			break;
		} else {
			cards.shuffle(input as u32);
			count += 1;
			print!("{}", JUMP);
		}
	}

	print!("{}", WIPE);
	if input > 0 && input < 4 {
		println!("Your number is: {}", cards.reveal());
	}
}

// vim: noexpandtab
