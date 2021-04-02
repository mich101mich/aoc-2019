use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/10.txt");
	let parsed = input
		.lines()
		.map(|l| l.chars().to_vec())
		//.map(parse)
		//.map(|l| scanf!(l, "{}", isize))
		.to_vec()
		//.sum::<isize>()
		;

	let h = parsed.len() as isize;
	let w = parsed[0].len() as isize;
	let (x, y) = (27, 19);
	let mut all = parsed
		.iter()
		.enumerate()
		.flat_map(|(y2, row)| {
			row.iter()
				.enumerate()
				.map(move |(x2, c)| (x2 as isize - x, y2 as isize - y, c))
		})
		.filter(|(_, _, c)| **c == '#')
		.map(|(x, y, _)| (x, y, (f32::atan2(x as f32, -y as f32) * 100_000.0) as isize))
		.map(|(x, y, v)| (x, y, if v < 0 { v + 700_000 } else { v }))
		.to_vec();
	all.sort_by_key(|(_, _, v)| *v);
	pv!(&all[0..20]);
	let mut next_iter = all.clone();

	let mut index = 1;
	let mut sorted = vec![];
	while !next_iter.is_empty() {
		let mut unused = vec![];

		while !next_iter.is_empty() {
			let mut curr = vec![];
			let v = next_iter[0].2;
			while !next_iter.is_empty() && next_iter[0].2 == v {
				curr.push(next_iter.remove(0));
			}
			curr.sort_by_key(|(x, y, _)| x.abs() + y.abs());
			sorted.push((curr[0].0 + x, curr[0].1 + y));
			if index == 200 {
				pv!((curr[0].0 + x) * 100 + curr[0].1 + y);
				//return;
			}
			index += 1;
			curr.remove(0);
			for v in curr {
				unused.push(v);
			}
			next_iter.sort_by_key(|(_, _, v)| *v);
		}
		next_iter = unused;
		next_iter.sort_by_key(|(_, _, v)| *v);
	}
	pv!(sorted[199]);
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/10.txt");
	let parsed = input
			.lines()
			.map(|l| l.chars().to_vec())
			//.map(parse)
			//.map(|l| scanf!(l, "{}", isize))
			.to_vec()
			//.sum::<isize>()
			;

	let h = parsed.len() as isize;
	let w = parsed[0].len() as isize;
	let mut max = 0;
	let mut max_v = (0, 0);
	for y in 0..h {
		for x in 0..w {
			if parsed[y as usize][x as usize] != '#' {
				continue;
			}
			let mut field = parsed.clone();
			let mut visible = 0;
			for delta in 0..w.max(h) {
				for dy in (-y).max(-delta)..(h - y).min(delta) {
					if dy.abs() > delta {
						continue;
					}
					for dx in (-x).max(-delta)..(w - x).min(delta) {
						if dx.abs() > delta || (dx == 0 && dy == 0) {
							continue;
						}
						let mut found = false;
						let mut x = x;
						let mut y = y;
						loop {
							x += dx;
							y += dy;
							if !(x >= 0 && x < w && y >= 0 && y < h) {
								break;
							}
							if field[y as usize][x as usize] == '#' {
								field[y as usize][x as usize] = '.';
								if !found {
									found = true;
									visible += 1;
								}
							}
						}
					}
				}
			}
			if visible > max {
				max = visible;
				max_v = (x, y);
			}
		}
	}
	pv!(max);
	pv!(max_v);
}
