#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days;
use days::*;

fn main() {
    day_15::run();
}
