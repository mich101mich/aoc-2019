use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/3.txt");
	// let input = ;
	let parsed = input
		.lines()
		.map(|l| l.split(',')
		.map(|l| (l.chars().next().unwrap(), i32::from_str(&l[1..]).unwrap_or_else(|_| panic!("failed to parse >{}<", &l[1..]))))
		//.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec())
		.to_vec()
		//.sum::<i32>()
		;
		let mut positions = HashSet::new();
	let mut pos = (0i32, 0i32);
	let mut distances = HashMap::new();
	let mut steps = 0;
	for (d, dist) in parsed[0].iter() {
		match d {
			'U' => {
				for _ in 0..*dist {
					pos.1 -= 1;
					positions.insert(pos);
					steps += 1;
					distances.entry(pos).or_insert(steps);
				}
			}
			'D' => {
				for _ in 0..*dist {
					pos.1 += 1;
					positions.insert(pos);
					steps += 1;
					distances.entry(pos).or_insert(steps);
				}
			}
			'L' => {
				for _ in 0..*dist {
					pos.0 -= 1;
					positions.insert(pos);
					steps += 1;
					distances.entry(pos).or_insert(steps);
				}
			}
			'R' => {
				for _ in 0..*dist {
					pos.0 += 1;
					positions.insert(pos);
					steps += 1;
					distances.entry(pos).or_insert(steps);
				}
			}
			_ => panic!("incalid"),
		}
	}
	let mut positions2 = HashSet::new();
	let mut pos = (0, 0);
	let mut distances2 = HashMap::new();
	let mut steps = 0;
	for (d, dist) in parsed[1].iter() {
		match d {
			'U' => {
				for _ in 0..*dist {
					pos.1 -= 1;
					positions2.insert(pos);
					steps += 1;
					distances2.entry(pos).or_insert(steps);
				}
			}
			'D' => {
				for _ in 0..*dist {
					pos.1 += 1;
					positions2.insert(pos);
					steps += 1;
					distances2.entry(pos).or_insert(steps);
				}
			}
			'L' => {
				for _ in 0..*dist {
					pos.0 -= 1;
					positions2.insert(pos);
					steps += 1;
					distances2.entry(pos).or_insert(steps);
				}
			}
			'R' => {
				for _ in 0..*dist {
					pos.0 += 1;
					positions2.insert(pos);
					steps += 1;
					distances2.entry(pos).or_insert(steps);
				}
			}
			_ => panic!("incalid"),
		}
	}
	let inter = positions
		.intersection(&positions2)
		.map(|pos| distances[pos] + distances2[pos])
		.min();
	pv!(inter);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/3.txt");
	// let input = ;
	let parsed = input
		.lines()
		.map(|l| l.split(',')
		.map(|l| (l.chars().next().unwrap(), i32::from_str(&l[1..]).unwrap_or_else(|_| panic!("failed to parse >{}<", &l[1..]))))
		//.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec())
		.to_vec()
		//.sum::<i32>()
		;
	let mut positions = HashSet::new();
	let mut pos = (0i32, 0i32);
	for (d, dist) in parsed[0].iter() {
		match d {
			'U' => {
				for _ in 0..*dist {
					pos.1 -= 1;
					positions.insert(pos);
				}
			}
			'D' => {
				for _ in 0..*dist {
					pos.1 += 1;
					positions.insert(pos);
				}
			}
			'L' => {
				for _ in 0..*dist {
					pos.0 -= 1;
					positions.insert(pos);
				}
			}
			'R' => {
				for _ in 0..*dist {
					pos.0 += 1;
					positions.insert(pos);
				}
			}
			_ => panic!("incalid"),
		}
	}
	let mut positions2 = HashSet::new();
	let mut pos = (0, 0);
	for (d, dist) in parsed[1].iter() {
		match d {
			'U' => {
				for _ in 0..*dist {
					pos.1 -= 1;
					positions2.insert(pos);
				}
			}
			'D' => {
				for _ in 0..*dist {
					pos.1 += 1;
					positions2.insert(pos);
				}
			}
			'L' => {
				for _ in 0..*dist {
					pos.0 -= 1;
					positions2.insert(pos);
				}
			}
			'R' => {
				for _ in 0..*dist {
					pos.0 += 1;
					positions2.insert(pos);
				}
			}
			_ => panic!("incalid"),
		}
	}
	let inter = positions
		.intersection(&positions2)
		.min_by_key(|(x, y)| x.abs() + y.abs());
	pv!(inter);
}
