use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/16.txt");
	// let input = ;

	let list = input.chars().map(|c| c as i64 - '0' as i64).to_vec();

	let offset = parse(&input[0..7]);

	let mut list = list
		.iter()
		.copied()
		.cycle()
		.take(list.len() * 10_000)
		.skip(offset as usize)
		.to_vec();

	for step in 0..100 {
		pv!(step);
		let mut new_list = list.clone();

		let mut sum = 0;
		for (i, v) in new_list.iter_mut().enumerate().rev() {
			sum += list[i];
			*v = sum % 10;
		}

		list = new_list;
		for v in list.iter().take(8) {
			print!("{}", v);
		}
		println!();
	}
	for v in list.iter().take(8) {
		print!("{}", v);
	}
	println!();
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/16.txt");
	// let input = ;

	let mut list = input.chars().map(|c| c as i64 - '0' as i64).to_vec();

	for _ in 0..100 {
		let mut new_list = list.clone();

		for (i, v) in new_list.iter_mut().enumerate() {
			let pattern = std::iter::repeat(0)
				.take(i + 1)
				.chain(std::iter::repeat(1).take(i + 1))
				.chain(std::iter::repeat(0).take(i + 1))
				.chain(std::iter::repeat(-1).take(i + 1))
				.to_vec();

			*v = list
				.iter()
				.zip(pattern.iter().cycle().skip(1))
				.map(|(v, n)| *v * n)
				.sum();
			*v = (*v % 10).abs();
		}

		list = new_list;
	}
	for v in list.iter() {
		print!("{}", v);
	}
	println!();
}
