# Advent of Code 2023
This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) programming puzzles, written in Rust 🦀.

This was my sixth Advent of Code and I am doing it "live"-ish.

# Goals
I've done a few of these now and I have a solid grasp on Rust so I'm not trying for too much here.
1. Have fun.
2. Solve the puzzles in a reasonably robust way.
    * Documentation is not a priority.
3. Write idiomatic Rust code wherever possible.
    * Latest stable compiler, no unsafe, clippy-clean.
4. See if I can get all solutions to run under 1 second. Might not try too hard to get over the line but it's something to shoot for.
5. Use external crates more rather than doing it myself with std. Itertools, regex, and maybe more.

# Building and running
This project uses the [Cargo AoC](https://github.com/gobanos/cargo-aoc) framework, which must be installed in order to build the program. Cargo AoC  makes it easy to download input files and supply them to the program, separate generators and solvers, and execute solutions selectively. It also provides an easy way to benchmark solutions.

All solutions can be tested and run with the usual cargo commands:
* `cargo test`
* `cargo run --release`

The solutions can be selectively run as follows:
* `cargo aoc -d D`, where D is replaced with the relevant day number (1-25)
* `cargo aoc -d D -p P`, same as above but replacing P with the relevant part number (1-2)

# Execution times
Time measurements were made using the command: `cargo aoc bench -d D`, where D is replaced with the relevant day number (1-25). The average measurement was used; in some cases it would be more accurate to use the fastest measurement as this best represents how the program is capable of performing, however in other cases there is significant variability in the run time due to the program itself (such as when using hashes, which internally have random seeds).

To automatically benchmark all days and record the results in the table below, use `python tools/auto_bench.py`

If the individual times are instead entered manually in this document, total time can be calculated using `python tools/time.py`

## Results

Day | Part | Time
:--:| :--: | :-------:

