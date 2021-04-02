use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/12.txt");
	// let input = ;
	let mut pos = input
		.lines()
		.map(|l| scanf!(l, "<x={}, y={}, z={}>", isize, isize, isize))
		.to_vec();

	let mut vel = vec![(0, 0, 0); pos.len()];
	let mut states = HashSet::with_capacity(50_000_000);

	// 0: 268296
	// 1: 22958
	// 2: 231614
	// lcm: 356,658,899,375,688

	for step in 0.. {
		if !states.insert((
			pos[0].2, pos[1].2, pos[2].2, pos[3].2, vel[0].2, vel[1].2, vel[2].2, vel[3].2,
		)) {
			pv!(step);
			break;
		}
		for (i, m1) in pos.iter().enumerate() {
			for (j, m2) in pos[i + 1..].iter().enumerate() {
				let j = j + i + 1;
				vel[j].2 += (m1.2 - m2.2).signum();
				vel[i].2 += (m2.2 - m1.2).signum();
			}
		}

		for (m, v) in pos.iter_mut().zip(vel.iter()) {
			m.2 += v.2;
		}
	}
}

#[allow(unused)]
pub fn part_one() {
	#[allow(unused_variables)]
	let input = include_str!("../input/12.txt");
	// let input = ;
	let mut pos = input
		.lines()
		.map(|l| scanf!(l, "<x={}, y={}, z={}>", isize, isize, isize))
		.to_vec();

	let mut vel = vec![(0, 0, 0); pos.len()];

	for _ in 0..1000 {
		for (i, m1) in pos.iter().enumerate() {
			for (j, m2) in pos[i + 1..].iter().enumerate() {
				let j = j + i + 1;
				if m1.0 > m2.0 {
					vel[i].0 -= 1;
					vel[j].0 += 1;
				} else if m1.0 < m2.0 {
					vel[i].0 += 1;
					vel[j].0 -= 1;
				}
				if m1.1 > m2.1 {
					vel[i].1 -= 1;
					vel[j].1 += 1;
				} else if m1.1 < m2.1 {
					vel[i].1 += 1;
					vel[j].1 -= 1;
				}
				if m1.2 > m2.2 {
					vel[i].2 -= 1;
					vel[j].2 += 1;
				} else if m1.2 < m2.2 {
					vel[i].2 += 1;
					vel[j].2 -= 1;
				}
			}
		}

		for (m, v) in pos.iter_mut().zip(vel.iter()) {
			m.0 += v.0;
			m.1 += v.1;
			m.2 += v.2;
		}
	}

	let energy: isize = pos
		.iter()
		.zip(vel.iter())
		.map(|(m, v)| {
			let pot = m.0.abs() + m.1.abs() + m.2.abs();
			let kin = v.0.abs() + v.1.abs() + v.2.abs();
			pot * kin
		})
		.sum();
	pv!(energy);
}
