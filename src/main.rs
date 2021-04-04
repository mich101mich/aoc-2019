#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_16;
}
use days::day_16;

fn main() {
    day_16::run();
}
