use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/9.txt");
	// let input = "104,1125899906842624,99";
	
	let mut code = IntProgram::new(input, vec![2]);
	int_code(&mut code, false);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/9.txt");
	// let input = "104,1125899906842624,99";
	
	let mut code = IntProgram::new(input, vec![1]);
	int_code(&mut code, false);
}
