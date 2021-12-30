pub mod guess {
	const DECK: usize = 52;
	const PLAY: usize = 21;
	const ROWS: usize = PLAY / 3;

	pub struct deck {
		pub arr_nums: [u32; PLAY],
	}
	impl deck {
	// private member functions.
		fn duplicated(&self, number: u32) -> bool {
			for element in self.arr_nums {
				if element == 0 {
					return false;
				} else if element == number {
					return true;
				}
			}
			return false;
		}
	// public member functions.
		pub fn generate() -> deck {
			let mut stack = deck { arr_nums: [0; PLAY], };
			let mut INDEX: usize = 0;

			while stack.arr_nums[PLAY - 1] == 0 {
				let seed: u32 = (rand::random::<u32>() % DECK as u32) + 1;
				if ! stack.duplicated(seed) {
					stack.arr_nums[INDEX] = seed;
					INDEX += 1;
				}
			}
			return stack;
		}
	}
}

// vim: noexpandtab
