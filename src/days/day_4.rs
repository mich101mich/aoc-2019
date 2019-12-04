use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/4.txt");
	// let input = ;
	let parsed = input
		.split('-')
		.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec();
	//.sum::<i32>();
	let min = parsed[0];
	let max = parsed[1];

	let count = (min..max)
		.into_par_iter()
		.map(|n| n.to_string())
		.filter(|n| {
			let mut adjacent = n.chars().zip(n.chars().skip(1));
			let mut m = 'x';
			let mut current_count = 0;
			let mut found = false;
			for c in n.chars() {
				if c == m {
					current_count += 1;
				} else {
					if current_count == 1 {
						found = true;
						break;
					}
					current_count = 0
				}
				m = c;
			}
			found = found || current_count == 1;
			found && adjacent.find(|(a, b)| a > b).is_none()
		})
		.count();
	pv!(count);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/4.txt");
	// let input = ;
	let parsed = input
		.split('-')
		.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec();
	//.sum::<i32>();
	let min = parsed[0];
	let max = parsed[1];

	let count = (min..max)
		.into_par_iter()
		.map(|n| n.to_string())
		.filter(|n| {
			let mut adjacent = n.chars().zip(n.chars().skip(1));
			adjacent.clone().find(|(a, b)| a == b).is_some()
				&& adjacent.find(|(a, b)| a > b).is_none()
		}).count();
	pv!(count);
}
