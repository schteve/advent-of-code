#![deny(clippy::allow_attributes)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate aoc_runner_derive;

use aoc_runner_derive::aoc_lib;

mod day_01;

aoc_lib! { year = 2024 }
