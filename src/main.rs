#![allow(unused_imports, clippy::while_let_on_iterator)]

#[macro_use]
mod utils;
mod days {
    pub mod day_25;
}
use days::day_25;

fn main() {
    day_25::run();
}
