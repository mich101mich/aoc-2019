DAY=$1
echo "pub use crate::utils::*;

pub fn run() {
	#[allow(unused_variables)]
	let input = include_str!("../input/$DAY.txt");
	let input = "";
	
	
}
" > src/days/day_$DAY.rs

echo "mod utils;
mod days;
use days::*;

fn main() {
    day_$DAY::run();
}
" > src/main.rs

echo "
pub mod day_$DAY;" >> src/days/mod.rs

touch src/input/$DAY.txt

code src/days/day_$DAY.rs src/input/1.txt 