#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_09;
}
use days::day_09;

fn main() {
    day_09::run();
}
