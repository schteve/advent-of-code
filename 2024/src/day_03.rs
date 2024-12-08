/*
    --- Day 3: Mull It Over ---
    "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

    The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

    The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

    It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

    However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

    For example, consider the following section of corrupted memory:

    xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

    Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?

    --- Part Two ---
    As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

    There are two new instructions you'll need to handle:

    The do() instruction enables future mul instructions.
    The don't() instruction disables future mul instructions.
    Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

    For example:

    xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

    This time, the sum of the results is 48 (2*4 + 8*5).

    Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?
*/

use regex::Regex;

enum Op {
    Mul(u32, u32),
    Do,
    Dont,
}

fn uncorrupt(mem: &str) -> Vec<Op> {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)()()|(don't)\(\)()()").unwrap();

    re.captures_iter(mem)
        .map(|c| c.extract())
        .map(|(_, [kind, a, b])| match kind {
            "mul" => Op::Mul(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()),
            "do" => Op::Do,
            "don't" => Op::Dont,
            _ => panic!("Invalid op found: {kind}"),
        })
        .collect()
}

fn computer_mul(ops: &[Op]) -> u64 {
    ops.iter()
        .map(|op| match op {
            Op::Mul(a, b) => (*a as u64) * (*b as u64),
            _ => 0,
        })
        .sum()
}

fn computer_all(ops: &[Op]) -> u64 {
    let mut enabled = true;
    ops.iter()
        .map(|op| match op {
            Op::Mul(a, b) => {
                if enabled {
                    (*a as u64) * (*b as u64)
                } else {
                    0
                }
            }
            Op::Do => {
                enabled = true;
                0
            }
            Op::Dont => {
                enabled = false;
                0
            }
        })
        .sum()
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let ops = uncorrupt(input);
    let value = computer_mul(&ops);
    assert_eq!(value, 174561379);
    value
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let ops = uncorrupt(input);
    let value = computer_all(&ops);
    assert_eq!(value, 106921067);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static EXAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_uncorrupt_mul() {
        let input = input_generator(EXAMPLE_INPUT_1);
        let ops = uncorrupt(&input);
        let value = computer_mul(&ops);
        assert_eq!(value, 161);
    }

    #[test]
    fn test_uncorrupt_all() {
        let input = input_generator(EXAMPLE_INPUT_2);
        let ops = uncorrupt(&input);
        let value = computer_all(&ops);
        assert_eq!(value, 48);
    }
}
