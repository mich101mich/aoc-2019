use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/15.txt");

	let mut code = IntProgram::new(input, vec![]);

	let mut area = vec![vec![-1; 50]; 50];
	let mut x = area[0].len() / 2;
	let mut y = area.len() / 2;

	for _ in 0..1_000_000 {
		let dir: isize = rand::thread_rng().gen_range(1, 5);
		code.inputs.push(dir);
		let res = int_code(&mut code, true).unwrap();
		match res {
			0 => match dir {
				1 => area[y - 1][x] = 1,
				2 => area[y + 1][x] = 1,
				3 => area[y][x - 1] = 1,
				4 => area[y][x + 1] = 1,
				_ => panic!("Invalid dir"),
			},
			c if c == 1 || c == 2 => {
				match dir {
					1 => y -= 1,
					2 => y += 1,
					3 => x -= 1,
					4 => x += 1,
					_ => panic!("Invalid dir"),
				}
				if c == 1 {
					area[y][x] = 0;
				} else {
					area[y][x] = 2;
				}
			}
			_ => panic!("Invalid res"),
		}
	}
	for row in area.iter() {
		for v in row.iter() {
			match *v {
				-1 => print!(" "),
				0 => print!("."),
				1 => print!("#"),
				2 => print!("@"),
				_ => panic!("Invalid"),
			}
		}
		println!();
	}

	let start = area
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.find(|(_, v)| **v == 2)
				.map(|(x, _)| (x as isize, y as isize))
		})
		.next()
		.unwrap();

	let goals = area
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.filter(|(_, v)| **v == 0)
				.map(move |(x, _)| (x as isize, y as isize))
		})
		.to_vec();

	let paths = dijkstra_search(
		get_all_neighbors,
		|_, _| 1,
		|(x, y)| {
			x >= 0
				&& y >= 0 && (x as usize) < area[0].len()
				&& (y as usize) < area.len()
				&& area[y as usize][x as usize] != 1
		},
		start,
		&goals,
	);

	pv!(paths.values().max_by_key(|p| p.cost));
}

fn get_all_neighbors(point: (isize, isize)) -> Box<dyn Iterator<Item = (isize, isize)>> {
	let (width, height) = (50, 50);

	let iter = [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)]
		.iter()
		.map(move |(dx, dy)| (point.0 as isize + dx, point.1 as isize + dy))
		.filter(move |(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height);

	Box::new(iter)
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/15.txt");

	let mut code = IntProgram::new(input, vec![]);

	let mut area = vec![vec![-1; 50]; 50];
	let mut x = area[0].len() / 2;
	let mut y = area.len() / 2;

	for _ in 0..100_000 {
		let dir: isize = rand::thread_rng().gen_range(1, 5);
		code.inputs.push(dir);
		let res = int_code(&mut code, true).unwrap();
		match res {
			0 => match dir {
				1 => area[y - 1][x] = 1,
				2 => area[y + 1][x] = 1,
				3 => area[y][x - 1] = 1,
				4 => area[y][x + 1] = 1,
				_ => panic!("Invalid dir"),
			},
			c if c == 1 || c == 2 => {
				match dir {
					1 => y -= 1,
					2 => y += 1,
					3 => x -= 1,
					4 => x += 1,
					_ => panic!("Invalid dir"),
				}
				if c == 1 {
					area[y][x] = 0;
				} else {
					area[y][x] = 2;
				}
			}
			_ => panic!("Invalid res"),
		}
	}
	for row in area.iter() {
		for v in row.iter() {
			match *v {
				-1 => print!(" "),
				0 => print!("."),
				1 => print!("#"),
				2 => print!("@"),
				_ => panic!("Invalid"),
			}
		}
		println!();
	}

	let start = (area[0].len() as isize / 2, area.len() as isize / 2);

	let goal = area
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.iter()
				.enumerate()
				.find(|(_, v)| **v == 2)
				.map(|(x, _)| (x as isize, y as isize))
		})
		.next()
		.unwrap();

	let path = a_star_search(
		get_all_neighbors,
		|_, _| 1,
		|(x, y)| {
			x >= 0
				&& y >= 0 && (x as usize) < area[0].len()
				&& (y as usize) < area.len()
				&& area[y as usize][x as usize] != 1
		},
		start,
		goal,
		|p| ((p.0 - goal.0).abs() + (p.1 - goal.1).abs()) as usize,
	);

	pv!(path);
}
