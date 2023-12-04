/*
    --- Day 3: Gear Ratios ---
    You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

    It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

    "Aaah!"

    You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

    The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

    The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

    Here is an example engine schematic:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

    Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

    --- Part Two ---
    The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

    You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

    Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

    The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

    This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

    Consider the same engine schematic again:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

    What is the sum of all of the gear ratios in your engine schematic?
*/

use std::ops::RangeInclusive;

use nom::AsChar;

struct Number {
    n: u32,
    x_span: RangeInclusive<i32>,
    y: i32,
}

impl Number {
    fn borders(&self) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        (
            (self.x_span.start() - 1)..=(self.x_span.end() + 1),
            (self.y - 1)..=(self.y + 1),
        )
    }
}

fn find_numbers(schematic: &[Vec<char>]) -> Vec<Number> {
    let mut numbers = Vec::new();

    let mut curr_num = None;
    let mut curr_start = None;
    let mut curr_end = None;
    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_dec_digit() {
                let digit = c.to_digit(10).unwrap();
                if curr_num.is_none() {
                    curr_num = Some(digit);
                    curr_start = Some(x as i32);
                    curr_end = Some(x as i32);
                } else {
                    curr_num = curr_num.map(|n| n * 10 + digit);
                    curr_end = Some(x as i32);
                }
            } else if let Some(n) = curr_num {
                let num = Number {
                    n,
                    x_span: (curr_start.unwrap()..=curr_end.unwrap()),
                    y: y as i32,
                };
                numbers.push(num);

                curr_num = None;
                curr_start = None;
                curr_end = None;
            }
        }

        if let Some(n) = curr_num {
            let pn = Number {
                n,
                x_span: (curr_start.unwrap()..=curr_end.unwrap()),
                y: y as i32,
            };
            numbers.push(pn);

            curr_num = None;
            curr_start = None;
            curr_end = None;
        }
    }

    numbers
}

struct Symbol {
    c: char,
    x: i32,
    y: i32,
}

fn find_symbols(schematic: &[Vec<char>]) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (y, row) in schematic.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if !c.is_dec_digit() && c != '.' {
                let sym = Symbol {
                    c,
                    x: x as i32,
                    y: y as i32,
                };
                symbols.push(sym);
            }
        }
    }

    symbols
}

fn find_part_numbers(schematic: &[Vec<char>]) -> Vec<u32> {
    let numbers = find_numbers(schematic);
    let symbols = find_symbols(schematic);

    let mut part_numbers = Vec::new();

    for num in numbers {
        let (x, y) = num.borders();
        for sym in &symbols {
            if x.contains(&sym.x) && y.contains(&sym.y) {
                part_numbers.push(num.n);
                break;
            }
        }
    }

    part_numbers
}

fn sum_part_numbers(schematic: &[Vec<char>]) -> u32 {
    find_part_numbers(schematic).into_iter().sum()
}

fn sum_gear_ratios(schematic: &[Vec<char>]) -> u32 {
    let numbers = find_numbers(schematic);
    let symbols = find_symbols(schematic);

    let mut gear_ratios = Vec::new();

    let mut adjacents = Vec::new();
    for sym in &symbols {
        if sym.c == '*' {
            adjacents.clear();

            for num in &numbers {
                let (x, y) = num.borders();
                if x.contains(&sym.x) && y.contains(&sym.y) {
                    adjacents.push(num.n);
                }
            }

            if adjacents.len() == 2 {
                gear_ratios.push(adjacents[0] * adjacents[1]);
            }
        }
    }

    gear_ratios.into_iter().sum()
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<char>]) -> u32 {
    let value = sum_part_numbers(input);
    assert_eq!(value, 532331);
    value
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<char>]) -> u32 {
    let value = sum_gear_ratios(input);
    assert_eq!(value, 82301120);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_sum_part_numbers() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = sum_part_numbers(&input);
        assert_eq!(value, 4361);
    }

    #[test]
    fn test_sum_gear_ratios() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = sum_gear_ratios(&input);
        assert_eq!(value, 467835);
    }
}
