DAY=$1
echo "use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!(\"../input/$DAY.txt\");
	// let input = "";
	
	let parsed = input
		.lines()
		//.map(|l| i32::from_str(l).unwrap_or_else(|_| panic!(\"failed to parse >{}<\", l)))
		//.map(|l| scanf!(l, \"{}\", i32))
		//.to_vec()
		//.sum::<i32>()
		;
	
	//pv!(parsed);
	
}" > src/days/day_$DAY.rs

echo "#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days;
use days::*;

fn main() {
    day_$DAY::run();
}" > src/main.rs

echo "
pub mod day_$DAY;" >> src/days/mod.rs

touch src/input/$DAY.txt

code src/days/day_$DAY.rs src/input/$DAY.txt 
