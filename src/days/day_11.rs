use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/11.txt");
	// let input = ;
	let mut code = IntProgram::new(input, vec![]);

	let mut hull = vec![vec![0; 100]; 100];
	let mut dir = Dir::Up;
	let (mut x, mut y) = (50, 50);

	hull[y][x] = 1;
	loop {
		code.inputs.push(hull[y][x]);
		if let Some(color) = int_code(&mut code, true) {
			hull[y][x] = color;
		} else {
			break;
		}
		let dir_change = int_code(&mut code, true).unwrap();
		if dir_change == 0 {
			dir = dir.counter_clockwise();
		} else {
			dir = dir.clockwise();
		}
		match dir {
			Dir::Up => y -= 1,
			Dir::Right => x += 1,
			Dir::Down => y += 1,
			Dir::Left => x -= 1,
		}
	}
	for row in &hull {
		for color in row {
			if *color == 1 {
				print!("#");
			} else {
				print!(" ");
			}
		}
		println!();
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/11.txt");
	// let input = ;
	let mut code = IntProgram::new(input, vec![]);

	let mut hull = vec![vec![0; 200]; 200];
	let mut dir = Dir::Up;
	let (mut x, mut y) = (100, 100);
	let mut painted = HashSet::new();

	loop {
		code.inputs.push(hull[y][x]);
		if let Some(color) = int_code(&mut code, true) {
			hull[y][x] = color;
			painted.insert((x, y));
		} else {
			break;
		}
		let dir_change = int_code(&mut code, true).unwrap();
		if dir_change == 0 {
			dir = dir.counter_clockwise();
		} else {
			dir = dir.clockwise();
		}
		match dir {
			Dir::Up => y -= 1,
			Dir::Right => x += 1,
			Dir::Down => y += 1,
			Dir::Left => x -= 1,
		}
	}
	pv!(painted.len());
}
