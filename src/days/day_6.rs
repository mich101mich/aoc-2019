use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/6.txt");
	// let input = "";
	let parsed = input
	.lines()
	.map(|l| l.split(')').to_vec())
	//.map(|l| i64::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
	//.map(|l| scanf!(l, "{}", i64))
		.to_vec()
		//.sum::<i64>()
		;

	let mut parent = HashMap::new();
	for x in parsed {
		parent.entry(x[0]).or_insert_with(|| vec![]).push(x[1]);
	}

	let path = dijkstra_search(
		|p| {
			let mut iter: Box<dyn Iterator<Item = &str>> = Box::new(
				parent
					.iter()
					.filter(move |(_, v)| v.contains(&p))
					.map(|(v, _)| *v),
			);
			if let Some(v) = parent.get(p) {
				iter = Box::new(iter.chain(v.iter().copied()));
			}
			iter
		},
		|_, _| 1,
		|_| true,
		"YOU",
		&["SAN"],
	);
	pv!(path["SAN"].cost - 2);
}

#[allow(unused)]
fn c(parent: &HashMap<&str, Vec<&str>>, current: &str) -> usize {
	let child = match parent.get(current) {
		Some(b) => b,
		None => return 0,
	};
	let mut cn = child.len();
	for chil in child.iter() {
		cn += c(parent, chil);
	}
	cn
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/6.txt");
	// let input = ;
	let parsed = input
	.lines()
	.map(|l| l.split(')').to_vec())
	//.map(|l| i64::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
	//.map(|l| scanf!(l, "{}", i64))
	.to_vec()
	//.sum::<i64>()
	;

	let mut count = 0;
	let mut parent = HashMap::new();
	for x in parsed {
		parent.entry(x[0]).or_insert_with(|| vec![]).push(x[1]);
	}

	pv!(count);
}
