use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/05.txt");

	let mut code = IntProgram::new(input, vec![5]);
	int_code(&mut code, false);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/05.txt");

	let mut code = IntProgram::new(input, vec![1]);
	int_code(&mut code, false);
}
