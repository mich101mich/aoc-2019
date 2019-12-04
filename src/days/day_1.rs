use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/1.txt");
	// let input = "";
	let parsed = input
		.lines()
		.map(|line| isize::from_str(line).unwrap())
		.map(|n| n / 3 - 2);
	let mut total = 0;
	for mut n in parsed {
		while n > 0 {
			total += n;
			n = n / 3 - 2;
		}
	}
	println!("parsed: {:?}", total);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/1.txt");
	// let input = "";
	let parsed: usize = input
		.lines()
		.map(|line| usize::from_str(line).unwrap())
		.map(|n| n / 3 - 2)
		.sum();
	println!("parsed: {:?}", parsed);
}
