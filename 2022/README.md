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

Day | Part | Time
:--:| :--: | :-------:
1   | 1    | 544.55 ns
1   | 2    | 671.41 ns
2   | 1    | 3.8260 us
2   | 2    | 2.1966 us
3   | 1    | 26.418 us
3   | 2    | 228.90 us
4   | 1    | 1.2454 us
4   | 2    | 1.1179 us
5   | 1    | 9.8009 us
5   | 2    | 26.362 us
6   | 1    | 4.6908 us
6   | 2    | 16.262 us
7   | 1    | 172.66 us
7   | 2    | 182.44 us
8   | 1    | 65.629 us
8   | 2    | 215.74 us
9   | 1    | 378.86 us
9   | 2    | 503.15 us
10  | 1    | 831.61 ns
10  | 2    | 1.3200 us
11  | 1    | 10.470 us
11  | 2    | 4.4792 ms
12  | 1    | 1.7031 ms
12  | 2    | 1.0767 ms
13  | 1    | 4.7133 us
13  | 2    | 103.11 us
14  | 1    | 216.63 us
14  | 2    | 3.4410 ms
15  | 1    | 540.85 ns
15  | 2    | 45.123 ms
16  | 1    | 700.17 ms
16  | 2    | 2.7871 s
17  | 1    | 1.2703 ms
17  | 2    | 1.2470 ms
18  | 1    | 554.95 us
18  | 2    | 1.4255 ms
19  | 1    | 5.4414 ms
19  | 2    | 4.4941 ms
20  | 1    | 60.348 ms
20  | 2    | 983.45 ms
21  | 1    | 309.79 us
21  | 2    | 1.5119 ms
22  | 1    | 1.4744 ms
22  | 2    | 612.70 us
23  | 1    | 8.2622 ms
23  | 2    | 458.50 ms
24  | 1    | 41.327 ms
24  | 2    | 129.68 ms
25  | 1    | 527.37 ns

Total: 5.2452 s
