use crate::utils::*;

#[allow(unused)]
pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/7.txt");
	//let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
	let mut parsed = input
		.lines()
		.flat_map(|l| l.split(','))
		.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!("failed to parse >{}<", l)))
		//.map(|l| scanf!(l, "{}", i32))
		.to_vec()
		//.sum::<i32>()
		;
	let mut programs = vec![(0, parsed); 5];

	let mut ids = (5i32..10).to_vec();
	let mut max = 0;
	for _ in 0..100_000 {
		let mut programs = programs.clone();
		ids.shuffle(&mut thread_rng());
		let mut value = 0;
		for (id, code) in ids.iter().zip(programs.iter_mut()) {
			if let Some(v) = int_code(code, &[*id, value]) {
				value = v;
			} else {
				panic!("end");
			}
		}
		let mut current = 0;
		let new_value = 'outer: loop {
			for code in programs.iter_mut() {
				if let Some(v) = int_code(code, &[value]) {
					value = v;
				} else {
					break 'outer current;
				}
			}
			current = value;
		};
		if new_value > max {
			max = new_value;
			pv!(new_value);
		}
	}
	pv!(max);
}
