use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/19.txt");

	let code = IntProgram::new(input, vec![]);

	let run = |x, y| int_code(&mut IntProgram::new(input, vec![x, y]), true).unwrap() != 0;

	'outer: for x in 100.. {
		let mut y = 0;
		while !run(x, y) {
			y += 1;
		}
		while !run(x + 99, y) {
			if !run(x, y + 99) {
				continue 'outer;
			}
			y += 1;
		}
		if !run(x, y + 99) {
			continue 'outer;
		}
		pv!(x * 10000 + y);
		break;
	}
	'outer2: for y in 100.. {
		let mut x = 0;
		while !run(x, y) {
			x += 1;
		}
		while !run(x, y + 99) {
			if !run(x + 99, y) {
				continue 'outer2;
			}
			x += 1;
		}
		if !run(x + 99, y) {
			continue 'outer2;
		}
		pv!(x * 10000 + y);
		break;
	}
}
//6840937
#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/19.txt");

	let mut code = IntProgram::new(input, vec![]);
	let mut count = 0;
	for y in 0..50 {
		for x in 0..50 {
			let mut code = code.clone();
			code.inputs.push(x);
			code.inputs.push(y);
			let res = int_code(&mut code, true).unwrap();
			count += res;
			print!("{}", res);
		}
		println!();
	}
	pv!(count);
}
