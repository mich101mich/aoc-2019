use crate::utils::*;
use std::collections::VecDeque;

fn find_in_maze(maze: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
	maze.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, v)))
		.filter(|(_, _, v)| **v == c)
		.map(|(x, y, _)| (x, y))
		.collect()
}

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/18.txt");
	// 	let input = "#############
	// #g#f.D#..h#l#
	// #F###e#E###.#
	// #dCba.#.BcIJ#
	// ######@######
	// #nK.L.#.G...#
	// #M###N#H###.#
	// #o#m..#i#jk.#
	// #############";
	// 	let input = "#############
	// #DcBa.#.GhKl#
	// #.###.#.#I###
	// #e#d##@##j#k#
	// ###C#.#.###J#
	// #fEbA.#.FgHi#
	// #############";

	// 2334

	let mut maze = input.lines().map(|l| l.chars().to_vec()).to_vec();

	let start = find_in_maze(&maze, '@')[0];
	maze[start.1][start.0] = '.';
	maze[start.1 - 1][start.0] = '#';
	maze[start.1][start.0 - 1] = '#';
	maze[start.1][start.0 + 1] = '#';
	maze[start.1 + 1][start.0] = '#';

	maze[start.1 - 1][start.0 - 1] = '@';
	maze[start.1 - 1][start.0 + 1] = '@';
	maze[start.1 + 1][start.0 - 1] = '@';
	maze[start.1 + 1][start.0 + 1] = '@';

	let keys_pos: HashMap<(usize, usize), char> = maze
		.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
		.filter(|(_, _, v)| v.is_alphabetic() && v.is_lowercase())
		.map(|(x, y, v)| ((x, y), v))
		.collect();

	let door_pos: HashMap<(usize, usize), char> = maze
		.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
		.filter(|(_, _, v)| v.is_alphabetic() && v.is_uppercase())
		.map(|(x, y, v)| ((x, y), v))
		.collect();

	let neigh = ManhattanNeighborhood::new(maze[0].len(), maze.len());
	let mut costs: HashMap<char, HashMap<char, (HashSet<char>, usize)>> = HashMap::new();
	let starts = [
		((start.0 - 1, start.1 - 1), '1'),
		((start.0 + 1, start.1 - 1), '2'),
		((start.0 - 1, start.1 + 1), '3'),
		((start.0 + 1, start.1 + 1), '4'),
	];
	for (&pos, &key) in keys_pos.iter().chain(starts.iter().map(|v| (&v.0, &v.1))) {
		let targets = keys_pos.keys().copied().filter(|p| *p != pos).to_vec();
		let found = dijkstra_search(
			|p| neigh.get_all_neighbors(p),
			|_, _| 1,
			|(x, y)| maze[y][x] != '#',
			pos,
			&targets,
		);
		let mut map = HashMap::new();

		for (pos, path) in found {
			let key2 = keys_pos[&pos];
			let cost = path.cost;
			let prec = path
				.path
				.iter()
				.filter_map(|p| door_pos.get(p))
				.copied()
				.map(to_lower)
				.collect();
			map.insert(key2, (prec, cost));
		}

		costs.insert(key, map);
	}

	pub fn dedup_insert(
		vector: &mut VecDeque<([char; 4], HashSet<char>, usize)>,
		element: ([char; 4], HashSet<char>, usize),
	) {
		for i in 0..vector.len() {
			if vector[i].0 == element.0 && vector[i].1 == element.1 {
				vector[i].2 = vector[i].2.min(element.2);
				return;
			}
			if vector[i].2 > element.2 {
				let mut j = i;
				while j < vector.len() {
					if vector[j].0 == element.0 && vector[j].1 == element.1 {
						vector.remove(j);
						break;
					}
					j += 1;
				}
				vector.insert(i, element);
				return;
			}
		}
		vector.push_back(element);
	}
	let mut next = VecDeque::with_capacity(100_000);

	let mut cache = HashSet::new();

	dedup_insert(&mut next, (['1', '2', '3', '4'], HashSet::new(), 0));

	let mut cnt = 0;
	while let Some((pos, keys, steps)) = next.pop_front() {
		cnt += 1;
		if cnt > 100 {
			println!("{} {}   {}", steps, keys.len(), next.len());
			cnt = 0;
		}

		if keys.len() == keys_pos.len() {
			pv!(steps);
			return;
		}
		let state = {
			let mut keys = keys.iter().copied().to_vec();
			keys.sort();
			(pos, keys)
		};
		if !cache.insert(state) {
			continue;
		}

		let mut found = false;
		for r in 0..4 {

			let candidates = costs[&pos[r]]
				.iter()
				.filter(|(k, _)| !keys.contains(k))
				.filter(|(_, (prec, _))| prec.is_subset(&keys));
	
			for (k, (_, cost)) in candidates {
				found = true;
				let steps = steps + cost;
				let mut keys = keys.clone();
				keys.insert(*k);
				let mut pos = pos;
				pos[r] = *k;
				dedup_insert(&mut next, (pos, keys, steps));
			}
		}
		if !found {
			pv!(found);
			pv!(steps);
			return;
		}
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/18.txt");

	// 4954

	let mut maze = input.lines().map(|l| l.chars().to_vec()).to_vec();

	let start = find_in_maze(&maze, '@')[0];
	maze[start.1][start.0] = '.';

	let keys_pos: HashMap<(usize, usize), char> = maze
		.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
		.filter(|(_, _, v)| v.is_alphabetic() && v.is_lowercase())
		.map(|(x, y, v)| ((x, y), v))
		.collect();

	let door_pos: HashMap<(usize, usize), char> = maze
		.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (x, y, *v)))
		.filter(|(_, _, v)| v.is_alphabetic() && v.is_uppercase())
		.map(|(x, y, v)| ((x, y), v))
		.collect();

	let neigh = ManhattanNeighborhood::new(maze[0].len(), maze.len());
	let mut costs: HashMap<char, HashMap<char, (HashSet<char>, usize)>> = HashMap::new();
	for (&pos, &key) in keys_pos.iter() {
		let targets = keys_pos.keys().copied().filter(|p| *p != pos).to_vec();
		let found = dijkstra_search(
			|p| neigh.get_all_neighbors(p),
			|_, _| 1,
			|(x, y)| maze[y][x] != '#',
			pos,
			&targets,
		);
		let mut map = HashMap::new();

		for (pos, path) in found {
			let key2 = keys_pos[&pos];
			let cost = path.cost;
			let prec = path
				.path
				.iter()
				.filter_map(|p| door_pos.get(p))
				.copied()
				.map(to_lower)
				.collect();
			map.insert(key2, (prec, cost));
		}

		costs.insert(key, map);
	}

	let starts = dijkstra_search(
		|p| neigh.get_all_neighbors(p),
		|_, _| 1,
		|(x, y)| maze[y][x] == '.',
		start,
		&keys_pos.keys().copied().to_vec()[..],
	);

	pub fn dedup_insert(
		vector: &mut VecDeque<(char, HashSet<char>, usize)>,
		element: (char, HashSet<char>, usize),
	) {
		for i in 0..vector.len() {
			if vector[i].0 == element.0 && vector[i].1 == element.1 {
				vector[i].2 = vector[i].2.min(element.2);
				return;
			}
			if vector[i].2 > element.2 {
				let mut j = i;
				while j < vector.len() {
					if vector[j].0 == element.0 && vector[j].1 == element.1 {
						vector.remove(j);
						break;
					}
					j += 1;
				}
				vector.insert(i, element);
				return;
			}
		}
		vector.push_back(element);
	}
	let mut next = VecDeque::with_capacity(50_000);

	for (k, path) in starts.iter() {
		let v = (
			keys_pos[k],
			std::iter::once(keys_pos[k]).collect::<HashSet<char>>(),
			path.cost,
		);
		dedup_insert(&mut next, v);
	}

	let mut cnt = 0;
	while let Some((pos, keys, steps)) = next.pop_front() {
		cnt += 1;
		if cnt > 100 {
			println!("{} {}", steps, next.len());
			cnt = 0;
		}
		let candidates = costs[&pos]
			.iter()
			.filter(|(k, _)| !keys.contains(k))
			.filter(|(_, (prec, _))| prec.is_subset(&keys));

		let mut found = false;
		for (k, (_, cost)) in candidates {
			found = true;
			let steps = steps + cost;
			let mut keys = keys.clone();
			keys.insert(*k);
			dedup_insert(&mut next, (*k, keys, steps));
		}

		// next.truncate(30_000);

		if !found {
			pv!(steps);
			return;
		}
	}
}

fn to_upper(c: char) -> char {
	c.to_uppercase().next().expect("to_upper")
}
fn to_lower(c: char) -> char {
	c.to_lowercase().next().expect("to_lower")
}
