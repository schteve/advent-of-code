# Advent of Code 2021
This repository contains my solutions for the [Advent of Code 2022](https://adventofcode.com/2022) programming puzzles, written in Rust 🦀.

This was my fifth Advent of Code and I am doing it "live", starting on December 1st and generally trying to complete each puzzle on the day it is released.

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

## Results

Day | Part | Time
:--:| :--: | :-------:
1   | 1    |
1   | 2    |
2   | 1    |
2   | 2    |
3   | 1    |
3   | 2    |
4   | 1    |
4   | 2    |
5   | 1    |
5   | 2    |
6   | 1    |
6   | 2    |
7   | 1    |
7   | 2    |
8   | 1    |
8   | 2    |
9   | 1    |
9   | 2    |
10  | 1    |
10  | 2    |
11  | 1    |
11  | 2    |
12  | 1    |
12  | 2    |
13  | 1    |
13  | 2    |
14  | 1    |
14  | 2    |
15  | 1    |
15  | 2    |
16  | 1    |
16  | 2    |
17  | 1    |
17  | 2    |
18  | 1    |
18  | 2    |
19  | 1    |
19  | 2    |
20  | 1    |
20  | 2    |
21  | 1    |
21  | 2    |
22  | 1    |
22  | 2    |
23  | 1    |
23  | 2    |
24  | 1    |
24  | 2    |
25  | 1    |

Total:
