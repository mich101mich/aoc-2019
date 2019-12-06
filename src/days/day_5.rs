use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/5.txt");
	// let input = ;
	let mut parsed = input
		.lines()
		.flat_map(|l| l.split(','))
		.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec()
		//.sum::<i32>()
		;

	int_code(&mut parsed);
	//pv!(parsed);
}
