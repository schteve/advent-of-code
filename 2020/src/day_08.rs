/*

*/

use common::{signed, trim_start};
use nom::{
    character::complete::{alpha1, char},
    multi::many1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone)]
pub enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

impl Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (op, arg)) = separated_pair(trim_start(alpha1), char(' '), signed)(input)?;

        let instruction = match op {
            "jmp" => Self::Jmp(arg),
            "acc" => Self::Acc(arg),
            "nop" => Self::Nop(arg),
            _ => panic!("Unknown operation"),
        };

        Ok((input, instruction))
    }

    fn transform(&self) -> Self {
        match *self {
            Self::Jmp(x) => Self::Nop(x),
            Self::Acc(x) => Self::Acc(x),
            Self::Nop(x) => Self::Jmp(x),
        }
    }
}

#[derive(Clone)]
pub struct GameConsole {
    program: Vec<Instruction>,
    ip: usize,
    accumulator: i32,
    executed: Vec<bool>,
}

impl GameConsole {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, program) = many1(Instruction::parser)(input)?;

        let program_len = program.len();
        Ok((
            input,
            Self {
                program,
                ip: 0,
                accumulator: 0,
                executed: vec![false; program_len],
            },
        ))
    }

    fn execute_next_instruction(&mut self) {
        match self.program[self.ip] {
            Instruction::Jmp(x) => {
                if x >= 0 {
                    self.ip += x as usize;
                } else {
                    self.ip -= (-x) as usize;
                }
            }
            Instruction::Acc(x) => {
                self.accumulator += x;
                self.ip += 1;
            }
            Instruction::Nop(_) => self.ip += 1,
        }
    }

    fn execute(&mut self) -> (bool, i32) {
        loop {
            if self.ip >= self.program.len() {
                return (false, self.accumulator);
            }
            if self.executed[self.ip] == true {
                return (true, self.accumulator);
            }

            self.executed[self.ip] = true;
            self.execute_next_instruction();
        }
    }

    fn fix_corruption(&mut self) -> i32 {
        loop {
            if self.ip >= self.program.len() {
                panic!("Program terminated normally!");
            }
            if self.executed[self.ip] == true {
                panic!("No corrupted instruction found!");
            }

            if matches!(
                self.program[self.ip],
                Instruction::Jmp(_) | Instruction::Nop(_)
            ) {
                let mut new_console = self.clone();
                new_console.program[self.ip] = self.program[self.ip].transform();

                let (is_infinite_loop, acc_value) = new_console.execute();
                if is_infinite_loop == false {
                    return acc_value;
                }
            }

            self.executed[self.ip] = true;
            self.execute_next_instruction();
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> GameConsole {
    GameConsole::parser(input).unwrap().1
}

#[aoc(day8, part1)]
pub fn part1(input: &GameConsole) -> i32 {
    let mut console = input.clone(); // aoc-runner can only give an immutable reference, so to get mutable I have to clone
    let (is_infinite_loop, acc_value) = console.execute();
    assert!(is_infinite_loop);
    assert_eq!(acc_value, 1939);
    acc_value
}

#[aoc(day8, part2)]
pub fn part2(input: &GameConsole) -> i32 {
    let mut console = input.clone(); // aoc-runner can only give an immutable reference, so to get mutable I have to clone
    let acc_value = console.fix_corruption();
    assert_eq!(acc_value, 2212);
    acc_value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_execute() {
        let mut console = input_generator(EXAMPLE_INPUT);
        let (is_infinite_loop, acc_value) = console.execute();
        assert_eq!(is_infinite_loop, true);
        assert_eq!(acc_value, 5);
    }

    #[test]
    fn test_fix_corruption() {
        let mut console = input_generator(EXAMPLE_INPUT);
        let acc_value = console.fix_corruption();
        assert_eq!(acc_value, 8);
    }
}
