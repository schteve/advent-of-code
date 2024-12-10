/*
    --- Day 7: Bridge Repair ---
    The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief isn't on this side of the bridge, though; maybe he's on the other side?

    When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.

    You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some young elephants were playing nearby and stole all the operators from their calibration equations! They could finish the calibrations if only someone could determine which test values could possibly be produced by placing any combination of operators into their calibration equations (your puzzle input).

    For example:

    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    Each line represents a single equation. The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.

    Operators are always evaluated left-to-right, not according to precedence rules. Furthermore, numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants holding two different types of operators: add (+) and multiply (*).

    Only three of the above equations can be made true by inserting operators:

    190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would give 29, but choosing * would give the test value (10 * 19 = 190).
    3267: 81 40 27 has two positions for operators. Of the four possible configurations of the operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated left-to-right)!
    292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
    The engineers just need the total calibration result, which is the sum of the test values from just the equations that could possibly be true. In the above example, the sum of the test values for the three equations listed above is 3749.

    Determine which equations could possibly be true. What is their total calibration result?

    --- Part Two ---
    The engineers seem concerned; the total calibration result you gave them is nowhere close to being within safety tolerances. Just then, you spot your mistake: some well-hidden elephants are holding a third type of operator.

    The concatenation operator (||) combines the digits from its left and right inputs into a single number. For example, 12 || 345 would become 12345. All operators are still evaluated left-to-right.

    Now, apart from the three equations that could be made true using only addition and multiplication, the above example has three more equations that can be made true by inserting operators:

    156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
    7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
    192: 17 8 14 can be made true using 17 || 8 + 14.
    Adding up all six test values (the three that could be made before using only + and * plus the new three that can now be made by also using ||) produces the new total calibration result of 11387.

    Using your new knowledge of elephant hiding spots, determine which equations could possibly be true. What is their total calibration result?
*/

pub struct Equation {
    goal: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn parser(line: &str) -> Self {
        let (goal_str, numbers_str) = line.split_once(':').unwrap();

        let goal = goal_str.parse::<u64>().unwrap();
        let numbers = numbers_str
            .split_ascii_whitespace()
            .map(|tok| tok.parse::<u64>().unwrap())
            .collect();

        Self { goal, numbers }
    }

    fn could_be_true2(&self) -> bool {
        // There's pow(2, ops) possibilities
        let max = 1 << (self.numbers.len() - 1);
        for op_code in 0..max {
            let mut total = self.numbers[0];
            for idx in 1..self.numbers.len() {
                if op_code & (1 << (idx - 1)) == 0 {
                    // add
                    total += self.numbers[idx];
                } else {
                    // mul
                    total *= self.numbers[idx];
                }
            }

            if total == self.goal {
                return true;
            }
        }
        false
    }

    fn could_be_true3(&self) -> bool {
        let max = 3u32.pow((self.numbers.len() - 1) as u32);
        for op_code in 0..max {
            let mut total = self.numbers[0];
            for idx in 1..self.numbers.len() {
                let modulus = 3u32.pow(idx as u32);
                let div = 3u32.pow((idx - 1) as u32);
                let digit = (op_code % modulus) / div;
                if digit == 0 {
                    // add
                    total += self.numbers[idx];
                } else if digit == 1 {
                    // mul
                    total *= self.numbers[idx];
                } else {
                    // concat
                    let concat = format!("{total}{}", self.numbers[idx]);
                    total = concat.parse::<u64>().unwrap();
                }
            }

            if total == self.goal {
                return true;
            }
        }
        false
    }
}

pub struct Calibration {
    equations: Vec<Equation>,
}

impl Calibration {
    fn parse(input: &str) -> Self {
        let equations = input.lines().map(Equation::parser).collect();
        Self { equations }
    }

    fn calib2_total(&self) -> u64 {
        self.equations
            .iter()
            .filter(|eq| eq.could_be_true2())
            .map(|eq| eq.goal)
            .sum()
    }

    fn calib3_total(&self) -> u64 {
        self.equations
            .iter()
            .filter(|eq| eq.could_be_true3())
            .map(|eq| eq.goal)
            .sum()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Calibration {
    Calibration::parse(input)
}

#[aoc(day7, part1)]
pub fn part1(input: &Calibration) -> u64 {
    let value = input.calib2_total();
    assert_eq!(value, 20665830408335);
    value
}

#[aoc(day7, part2)]
pub fn part2(input: &Calibration) -> u64 {
    let value = input.calib3_total();
    assert_eq!(value, 354060705047464);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_calib2_total() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.calib2_total();
        assert_eq!(value, 3749);
    }

    #[test]
    fn test_calib3_total() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.calib3_total();
        assert_eq!(value, 11387);
    }
}
