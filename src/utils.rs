#![allow(unused)]

pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::io::Write;
pub use std::str::FromStr;

macro_rules! pv {
	($var: expr) => {
		println!("{}: {:?}", stringify!($var), $var)
	};
}

pub fn int_code(memory: &mut [i32]) -> usize {
	let mut index = 0;
	let mut inst_count = 0;
	loop {
		let op = memory[index];
		let p0 = (op / 100) % 10;
		let p1 = (op / 1000) % 10;
		let p2 = (op / 10000) % 10;
		let op = op % 100;
		match op {
			1 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if p2 == 0 {
					memory[memory[index + 3] as usize] = a + b;
				} else {
					panic!("output is value");
				};
				index += 4;
			}
			2 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if p2 != 0 {
					panic!("output is value");
				}
				memory[memory[index + 3] as usize] = a * b;
				index += 4;
			}
			3 => {
				memory[memory[index + 1] as usize] = 5;
				index += 2;
			}
			4 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				println!("{}", a);
				index += 2;
			}
			5 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if a != 0 {
					index = b as usize;
				} else {
					index += 3;
				}
			}
			6 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if a == 0 {
					index = b as usize;
				} else {
					index += 3;
				}
			}
			7 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if p2 != 0 {
					panic!("output is value");
				}
				memory[memory[index + 3] as usize] = (a < b) as i32;
				index += 4;
			}
			8 => {
				let a = if p0 == 0 {
					memory[memory[index + 1] as usize]
				} else {
					memory[index + 1]
				};
				let b = if p1 == 0 {
					memory[memory[index + 2] as usize]
				} else {
					memory[index + 2]
				};
				if p2 != 0 {
					panic!("output is value");
				}
				memory[memory[index + 3] as usize] = (a == b) as i32;
				index += 4;
			}
			99 => break,
			_ => panic!("invalid opcode"),
		}
		inst_count += 1;
	}
	inst_count
}

macro_rules! scanf {
    ( $instr:expr, $fmt:expr, $($($args:tt)::*),* ) => {
        {
            let mut res = scan_fmt::parse::scan( $instr, $fmt ) ;
            ($(scan_fmt::scan_fmt_help!(wrap res,$($args)::*).unwrap_or_else(|| panic!("Failed to parse {}", stringify!($expr)))),*)
        }
    };
}

pub trait IterExt<T> {
	fn to_vec(self) -> Vec<T>;
}
impl<T, I: Iterator<Item = T>> IterExt<T> for I {
	fn to_vec(self) -> Vec<T> {
		self.collect()
	}
}

pub fn neighbors(
	(x, y): (usize, usize),
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	let mut n = vec![];
	if y > 0 {
		n.push((x, y - 1));
	}
	if x > 0 {
		n.push((x - 1, y));
	}
	if x < width - 1 {
		n.push((x + 1, y));
	}
	if y < height - 1 {
		n.push((x, y + 1));
	}
	n.into_iter()
}

pub fn diff(a: usize, b: usize) -> usize {
	(a as isize - b as isize).abs() as usize
}
pub fn diff_i(a: isize, b: isize) -> usize {
	(a - b).abs() as usize
}

pub fn manhattan(p1: (usize, usize), p2: (usize, usize)) -> usize {
	diff(p1.0, p2.0) + diff(p1.1, p2.1)
}

pub fn manhattan_3d(p1: (usize, usize, usize), p2: (usize, usize, usize)) -> usize {
	diff(p1.0, p2.0) + diff(p1.1, p2.1) + diff(p1.2, p2.2)
}
pub fn manhattan_3d_i(p1: (isize, isize, isize), p2: (isize, isize, isize)) -> usize {
	diff_i(p1.0, p2.0) + diff_i(p1.1, p2.1) + diff_i(p1.2, p2.2)
}

pub fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
	vec![vec![value; width]; height]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}
pub use Dir::*;

pub type Cost = usize;

#[derive(Debug, Clone)]
pub struct Path<P> {
	pub path: Vec<P>,
	pub cost: Cost,
}

impl<P> Path<P> {
	pub fn new(path: Vec<P>, cost: Cost) -> Path<P> {
		Path { path, cost }
	}
	pub fn append(&mut self, node: P, cost: Cost) -> &mut Self {
		self.path.push(node);
		self.cost += cost;
		self
	}
}

use std::ops::*;

impl<P> Index<usize> for Path<P> {
	type Output = P;
	fn index(&self, index: usize) -> &P {
		&self.path[index]
	}
}

pub fn ordered_insert<T, V, F>(vector: &mut Vec<T>, element: T, get_value: F)
where
	T: std::fmt::Debug,
	V: Ord,
	F: Fn(&T) -> V,
{
	let value = get_value(&element);
	for i in 0..vector.len() {
		if get_value(&vector[i]) <= value {
			vector.insert(i, element);
			return;
		}
	}
	vector.push(element);
}

pub fn a_star_search<Id, GetNeighbors, NeighborIter, GetCost, IsWalkable, Heuristic>(
	get_all_neighbors: GetNeighbors,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goal: Id,
	heuristic: Heuristic,
) -> Option<Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + std::fmt::Debug,
	GetNeighbors: Fn(Id) -> NeighborIter,
	NeighborIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	Heuristic: Fn(Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	if start == goal {
		return Some(Path::new(vec![start, start], 0));
	}
	let mut visited = HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	'search: while let Some((current_id, _)) = next.pop() {
		if current_id == goal {
			break 'search;
		}
		let current_cost = visited[&current_id].0;

		for other_id in get_all_neighbors(current_id) {
			let other_cost = current_cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) && other_id != goal {
				continue;
			}

			let heuristic = heuristic(other_id);

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(
					&mut next,
					(other_id, other_cost + heuristic),
					|&(_, cost)| cost,
				);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	if !visited.contains_key(&goal) {
		return None;
	}

	let steps = {
		let mut steps = vec![];
		let mut current = goal;

		while current != start {
			steps.push(current);
			let (_, prev) = visited[&current];
			current = prev;
		}
		steps.push(start);
		steps.reverse();
		steps
	};

	Some(Path::new(steps, visited[&goal].0))
}

pub fn dijkstra_search<Id, GetNeighbors, NeighborIter, GetCost, IsWalkable>(
	get_all_neighbors: GetNeighbors,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goals: &[Id],
) -> HashMap<Id, Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + ::std::fmt::Debug,
	GetNeighbors: Fn(Id) -> NeighborIter,
	NeighborIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	let mut visited = ::std::collections::HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	let mut remaining_goals = goals.to_vec();

	let mut goal_costs = HashMap::with_capacity(goals.len());

	while let Some((current_id, _)) = next.pop() {
		let cost = visited[&current_id].0;

		for &goal_id in remaining_goals.iter() {
			if current_id == goal_id {
				goal_costs.insert(goal_id, cost);
			}
		}
		remaining_goals.retain(|&id| id != current_id);
		if remaining_goals.is_empty() {
			break;
		}

		for other_id in get_all_neighbors(current_id) {
			let other_cost = cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) {
				let mut is_goal = false;
				for &goal_id in remaining_goals.iter() {
					if other_id == goal_id {
						is_goal = true;
					}
				}
				if !is_goal {
					continue;
				}
			}

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(&mut next, (other_id, other_cost), |&(_, cost)| cost);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	let mut goal_data = HashMap::with_capacity(goal_costs.len());

	for (&goal, &cost) in goal_costs.iter() {
		let steps = {
			let mut steps = vec![];
			let mut current = goal;

			while current != start {
				steps.push(current);
				let (_, prev) = visited[&current];
				current = prev;
			}
			steps.push(start);
			steps.reverse();
			steps
		};
		goal_data.insert(goal, Path::new(steps, cost));
	}

	goal_data
}
