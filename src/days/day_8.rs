use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/8.txt");
	// let input = "";

	let parsed = input
		.chars()
		//.map(|l| parse(l))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec()
		//.sum::<i32>()
		;
	let w = 25;
	let h = 6;

	let mut image = vec![vec!['2'; w]; h];

	let mut offset = 0;
	for layer in 0..parsed.len() / (w * h) {
		for y in 0..h {
			for x in 0..w {
				if image[y][x] == '2' {
					image[y][x] = parsed[offset + y * w + x];
				}
			}
		}
		offset += w * h;
	}
	
	for y in 0..h {
		for x in 0..w {

			print!("{}", if image[y][x] == '1' {'1'}else {' '});
		}
		println!();
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/8.txt");
	// let input = "";

	let parsed = input
		.chars()
		//.map(|l| parse(l))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec()
		//.sum::<i32>()
		;
	let w = 25; let h = 6;

	let mut min = 999999999usize;
	let mut min_layer = 0;
	let mut offset = 0;
	for layer in 0..parsed.len() / (w * h) {
		let mut sum = 0;
		for i in 0..(w * h) {
			sum += (parsed[i + offset] == '0') as usize;
		}
		if sum < min {
			min = sum;
			min_layer = layer;
		}
		offset += w * h;
	}

	offset = min_layer * w * h;
	let mut ones = 0;
	let mut twos = 0;
	for i in 0..(w * h) {
		ones += (parsed[i + offset] == '1') as usize;
		twos += (parsed[i + offset] == '2') as usize;
	}

	pv!(ones * twos);
	
}
