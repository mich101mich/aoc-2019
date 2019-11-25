#![allow(dead_code)]

pub fn manhattan((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
	((ax as isize - bx as isize).abs() + (ay as isize - by as isize).abs()) as usize
}

pub fn moore((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
	isize::max(
		(ax as isize - bx as isize).abs(),
		(ay as isize - by as isize).abs(),
	) as usize
}
