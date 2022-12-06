/*
    --- Day 5: Supply Stacks ---
    The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

    The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

    The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

    They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

        [D]
    [N] [C]
    [Z] [M] [P]
    1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

    Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

    [D]
    [N] [C]
    [Z] [M] [P]
    1   2   3
    In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

            [Z]
            [N]
        [C] [D]
        [M] [P]
    1   2   3
    Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

            [Z]
            [N]
    [M]     [D]
    [C]     [P]
    1   2   3
    Finally, one crate is moved from stack 1 to stack 2:

            [Z]
            [N]
            [D]
    [C] [M] [P]
    1   2   3
    The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

    After the rearrangement procedure completes, what crate ends up on top of each stack?

    --- Part Two ---
    As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

    Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

    The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

    Again considering the example above, the crates begin in the same configuration:

        [D]
    [N] [C]
    [Z] [M] [P]
    1   2   3
    Moving a single crate from stack 2 to stack 1 behaves the same as before:

    [D]
    [N] [C]
    [Z] [M] [P]
    1   2   3
    However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

            [D]
            [N]
        [C] [Z]
        [M] [P]
    1   2   3
    Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

            [D]
            [N]
    [C]     [Z]
    [M]     [P]
    1   2   3
    Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

            [D]
            [N]
            [Z]
    [M] [C] [P]
    1   2   3
    In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

    Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use itertools::Itertools;
use regex::{Captures, Regex};

#[derive(Clone)]
pub struct Procedure {
    num: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct Supplies {
    stacks: Vec<Vec<char>>,
    procedures: Vec<Procedure>,
}

impl Supplies {
    fn rearrange1(&mut self) {
        for procedure in &self.procedures {
            for _ in 0..procedure.num {
                let tmp = self.stacks[procedure.from - 1].pop().unwrap();
                self.stacks[procedure.to - 1].push(tmp);
            }
        }
    }

    fn rearrange2(&mut self) {
        for procedure in &self.procedures {
            let start = self.stacks[procedure.from - 1].len() - procedure.num;
            #[allow(clippy::needless_collect)] // Borrow checker forbids it
            let tmp: Vec<char> = self.stacks[procedure.from - 1].drain(start..).collect();
            self.stacks[procedure.to - 1].extend(tmp.into_iter());
        }
    }

    fn read_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Supplies {
    let input = input.replace("\r\n", "\n");
    let (crates, directions) = input.split_once("\n\n").unwrap();

    let rows: Vec<Vec<Option<char>>> = crates
        .lines()
        .rev() // Easier to do this bottom up
        .skip(1) // Column designators
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunk| match chunk.nth(1) {
                    Some(' ') | None => None,
                    x => x,
                })
                .collect()
        })
        .collect();

    let stacks = (0..rows[0].len())
        .map(|i| rows.iter().filter_map(|row| row[i]).collect())
        .collect();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let cap_to_usize = |caps: &Captures, n| caps.get(n).unwrap().as_str().parse::<usize>().unwrap();
    let procedures = directions
        .lines()
        .filter_map(|line| re.captures(line))
        .map(|captures| Procedure {
            num: cap_to_usize(&captures, 1),
            from: cap_to_usize(&captures, 2),
            to: cap_to_usize(&captures, 3),
        })
        .collect();

    Supplies { stacks, procedures }
}

#[aoc(day5, part1)]
pub fn part1(input: &Supplies) -> String {
    let mut input = input.clone();
    input.rearrange1();
    let top = input.read_top();
    assert_eq!(top, "FJSRQCFTN");
    top
}

#[aoc(day5, part2)]
pub fn part2(input: &Supplies) -> String {
    let mut input = input.clone();
    input.rearrange2();
    let top = input.read_top();
    assert_eq!(top, "CJVLJQPHS");
    top
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "    [D]    '
[N] [C]    '
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_rearrange1() {
        let mut input = input_generator(EXAMPLE_INPUT);
        input.rearrange1();
        let top = input.read_top();
        assert_eq!(top, "CMZ");
    }

    #[test]
    fn test_rearrange2() {
        let mut input = input_generator(EXAMPLE_INPUT);
        input.rearrange2();
        let top = input.read_top();
        assert_eq!(top, "MCD");
    }
}
