use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/04.txt");

	let (min, max) = scanf!(input, "{}-{}", usize, usize).unwrap();

	let count = (min..=max)
		.filter(|i| {
			let mut n = *i;
			let mut prev = n % 10;
			n /= 10;
			let mut same = false;
			let mut match_count = 0;
			for _ in 0..5 {
				let last_digit = n % 10;
				n /= 10;
				if last_digit == prev {
					match_count += 1;
				} else {
					same = same || match_count == 1;
					match_count = 0;
				}
				if prev < last_digit {
					return false;
				}
				prev = last_digit
			}
			same = same || match_count == 1;
			same
		})
		.count();
	pv!(count);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/04.txt");

	let (min, max) = scanf!(input, "{}-{}", usize, usize).unwrap();

	let count = (min..=max)
		.filter(|i| {
			let mut n = *i;
			let mut prev = n % 10;
			n /= 10;
			let mut same = false;
			for _ in 0..5 {
				let last_digit = n % 10;
				n /= 10;
				same = same || last_digit == prev;
				if prev < last_digit {
					return false;
				}
				prev = last_digit
			}
			same
		})
		.count();
	pv!(count);
}
