use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/9.txt");
	// let input = "104,1125899906842624,99";
	
	let mut code = IntProgram::new(input);
	pv!(int_code(&mut code, &[2], false));
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/9.txt");
	// let input = "104,1125899906842624,99";
	
	let mut code = IntProgram::new(input);
	pv!(int_code(&mut code, &[1], false));
}
