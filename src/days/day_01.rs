use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/01.txt");

	let parsed = input.lines().map(parse).map(|n| n / 3 - 2);

	let mut total = 0;
	for mut n in parsed {
		while n > 0 {
			total += n;
			n = n / 3 - 2;
		}
	}
	pv!(total);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/01.txt");

	let parsed: usize = input.lines().map(parse_u).map(|n| n / 3 - 2).sum();

	pv!(parsed);
}
