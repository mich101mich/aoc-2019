#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_06;
}
use days::day_06;

fn main() {
    day_06::run();
}
