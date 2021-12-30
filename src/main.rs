#![warn(deprecated_in_future)]
#![forbid(deprecated)]
#![allow(non_camel_case_types, non_snake_case)]

mod guess;
use guess::guess::deck;

fn main() {
	let cards = deck::generate();
	println!("{:?}", cards.arr_nums);
}

// vim: noexpandtab
