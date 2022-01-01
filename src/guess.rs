pub mod guess {
	use std::fmt;

	const DECK: usize = 52;
	const PLAY: usize = 21;
	const ROWS: usize = PLAY / 3;

	pub struct deck {
		arr_nums: [u32; PLAY],
	}
	impl fmt::Display for deck {
		fn fmt(&self, F: &mut fmt::Formatter) -> fmt::Result {
			let mut out: String = String::from("");

			for each in 0..PLAY {
				out += &format!("{:>6}", self.arr_nums[each]);
				if (each + 1) % 3 == 0 {
					out += "\n";
				}
			}

			return write!(F, "{}", out);
		}
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
		pub fn shuffle(&mut self, S: u32) {
			let mut arr_slice_1: [u32; ROWS] = [0; ROWS];
			let mut arr_slice_2: [u32; ROWS] = [0; ROWS];
			let mut arr_slice_3: [u32; ROWS] = [0; ROWS];

			let mut index_arr_1: usize = ROWS;
			let mut index_arr_2: usize = ROWS;
			let mut index_arr_3: usize = ROWS;

			for each in 0..PLAY {
				match each % 3 {
					0	=> {
						index_arr_1 -= 1;
						arr_slice_1[index_arr_1] = self.arr_nums[each];
					}
					1	=> {
						index_arr_2 -= 1;
						arr_slice_2[index_arr_2] = self.arr_nums[each];
					}
					2	=> {
						index_arr_3 -= 1;
						arr_slice_3[index_arr_3] = self.arr_nums[each];
					}
					_	=> continue, // TODO: Error calculation handling here.
				}
			}

			let mut index_arr_all: usize = 0;
			match S {
				1	=> {
					for i in arr_slice_1 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_2 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_3 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
				}
				2	=> {
					for i in arr_slice_2 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_3 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_1 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
				}
				3	=> {
					for i in arr_slice_3 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_1 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
					for i in arr_slice_2 {
						self.arr_nums[index_arr_all] = i;
						index_arr_all += 1;
					}
				}
				_	=> panic!("{} is unvalid", S), // TODO: Wrong S value handling here.
			}
		}
		pub const fn reveal(&self) -> u32 {
			return self.arr_nums[5];
		}
	}
}

// vim: noexpandtab
