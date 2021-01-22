# Advent of Code 2020
This repository contains my solutions for the [Advent of Code 2020](https://adventofcode.com/2020) programming puzzles, written in Rust 🦀.

This was my third Advent of Code and the first one I did "live", starting on December 1st and generally trying to complete each puzzle on the day it was released.

I used this as an opportunity to refine my Rust language skills. My goals were:
1. Solve the puzzles in a reasonably robust way.
    * Create solutions that were "general enough", such that any reasonable input would be solved by the program. In some cases it would be prohibitively difficult to make a truly general solution and the input data showed clear intent to not require one, then some shortcuts were acceptable.
    * Create programs with good structure. Use structs, mods, and composition.
    * Follow best practices by writing unit tests wherever it benefited the process of solving the puzzle. Typically, any important functionality with non-trivial edge cases would be tested.
    * Documentation was not a priority. I made no effort to add comments describing the overall solution, nor the individual functions. Comments were added only where it helped the writing process. In a real life production environment comments and good documentation would of course be required.
2. Write idiomatic Rust code wherever possible.
    * The code compiles with no warnings, has no clippy warnings (see below), and conforms to the Rust formatting guidelines.
    * Use only features of the latest version of stable Rust.
    * No unsafe code.
3. Learn or practice features of the Rust language where the solutions present an opportunity, even if it's not the ideal fit for the situation.
4. Create efficient solutions. In the end I set a goal to have all solutions execute within 2 seconds total, which I achieved (see below). I usually chose good structure over optimization, but for days when the solution took a long time to run the optimization became a priority.
5. Practice writing parsers using the nom crate.
6. Use few external crates, relying mainly on the standard library.

# Building and running
This project uses the [Cargo AoC](https://github.com/gobanos/cargo-aoc) framework, which must be installed in order to build the program. Cargo AoC  makes it easy to download input files and supply them to the program, separate generators and solvers, and execute solutions selectively. It also provides an easy way to benchmark solutions.

All solutions can be tested and run with the usual cargo commands:
* `cargo test`
* `cargo run --release`

The solutions can be selectively run as follows:
* `cargo aoc -d D`, where D is replaced with the relevant day number (1-25)
* `cargo aoc -d D -p P`, same as above but replacing P with the relevant part number (1-2)

## Clippy
The clippy linter does not produce any warnings on the code at the default warning levels, with few exceptions where it is suppressed:
* `clippy::bool_comparison` and `clippy::needless_bool` - I find it far more readable to explicitly write booleans in most places they are used

## Commit hook
Each commit is checked with the following commands:
* `cargo fmt -- --check`
* `cargo test`
* `cargo clean; cargo clippy -- -Dwarnings`

# Execution times
After completing the puzzles I measured the execution times of each solution and performed some optimization in order to reduce total execution time. This year most of my initial solutions completed in less than 1 ms, so the bulk of the optimization was done on those days that were significantly longer.

Time measurements were made using the command: `cargo aoc bench -d 1`, where 1 is replaced with the relevant day number. The average measurement was used; in some cases it would be more accurate to use the fastest measurement as this best represents how the program is capable of performing, however in other cases there is significant variability in the run time due to the program itself (such as when using hashes, which internally have random seeds).

All measurements were taken on my PC built in 2012; I suspect using modern hardware would further reduce the times significantly.

**Total: 1.9098 s**

Day | Part | Time
:--:| :--: |----------
1   | 1    | 7.3002 us
1   | 2    | 2.1739 ms
2   | 1    | 23.673 us
2   | 2    | 35.255 us
3   | 1    | 13.758 us
3   | 2    | 58.394 us
4   | 1    | 1.3147 us
4   | 2    | 21.324 us
5   | 1    | 35.156 us
5   | 2    | 56.560 us
6   | 1    | 483.23 us
6   | 2    | 386.04 us
7   | 1    | 651.53 us
7   | 2    | 20.468 us
8   | 1    | 1.0927 us
8   | 2    | 50.330 us
9   | 1    | 44.079 us
9   | 2    | 45.599 us
10  | 1    | 1.8680 us
10  | 2    | 2.2301 us
11  | 1    | 25.777 ms
11  | 2    | 27.809 ms
12  | 1    | 5.9009 us
12  | 2    | 5.2090 us
13  | 1    | 258.41 ns
13  | 2    | 5.4489 us
14  | 1    | 32.926 us
14  | 2    | 12.164 ms
15  | 1    | 5.0393 us
15  | 2    | 607.50 ms
16  | 1    | 60.266 us
16  | 2    | 353.91 us
17  | 1    | 2.0533 ms
17  | 2    | 37.704 ms
18  | 1    | 363.49 us
18  | 2    | 417.24 us
19  | 1    | 29.563 ms
19  | 2    | 135.93 ms
20  | 1    | 575.38 us
20  | 2    | 4.0649 ms
21  | 1    | 922.13 us
21  | 2    | 764.47 us
22  | 1    | 14.273 us
22  | 2    | 509.71 ms
23  | 1    | 1.8879 us
23  | 2    | 298.77 ms
24  | 1    | 97.137 us
24  | 2    | 94.102 ms
25  | 1    | 116.89 ms
