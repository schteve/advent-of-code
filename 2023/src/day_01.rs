/*
    --- Day 1: Trebuchet?! ---
    Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

    You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

    You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

    As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

    The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the calibration values?

    --- Part Two ---
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

    Equipped with this new information, you now need to find the real first and last digit on each line. For example:

    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

    What is the sum of all of the calibration values?
*/

fn calibration_value_digit(line: &str) -> u32 {
    assert!(line.is_ascii());

    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    let it = line.chars().filter_map(|c| c.to_digit(10));
    for digit in it {
        if first.is_none() {
            first = Some(digit);
        }
        last = Some(digit);
    }

    first.unwrap() * 10 + last.unwrap()
}

fn calibration_sum_digit(lines: &[String]) -> u32 {
    lines.iter().map(|line| calibration_value_digit(line)).sum()
}

fn calibration_value_all(line: &str) -> u32 {
    assert!(line.is_ascii());

    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    let mut slice = line;
    while !slice.is_empty() {
        let digit = match slice {
            s if s.starts_with('0') => Some(0),
            s if s.starts_with('1') || s.starts_with("one") => Some(1),
            s if s.starts_with('2') || s.starts_with("two") => Some(2),
            s if s.starts_with('3') || s.starts_with("three") => Some(3),
            s if s.starts_with('4') || s.starts_with("four") => Some(4),
            s if s.starts_with('5') || s.starts_with("five") => Some(5),
            s if s.starts_with('6') || s.starts_with("six") => Some(6),
            s if s.starts_with('7') || s.starts_with("seven") => Some(7),
            s if s.starts_with('8') || s.starts_with("eight") => Some(8),
            s if s.starts_with('9') || s.starts_with("nine") => Some(9),
            _ => None,
        };

        if let Some(d) = digit {
            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
        }

        let (_, remaining) = slice.split_at(1);
        slice = remaining;
    }

    first.unwrap() * 10 + last.unwrap()
}

fn calibration_sum_all(lines: &[String]) -> u32 {
    lines.iter().map(|line| calibration_value_all(line)).sum()
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    let value = calibration_sum_digit(input);
    assert_eq!(value, 55123);
    value
}

#[aoc(day1, part2)]
pub fn part2(input: &[String]) -> u32 {
    let value = calibration_sum_all(input);
    assert_eq!(value, 55260);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static EXAMPLE_INPUT_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_calibration_sum_digit() {
        let input = input_generator(EXAMPLE_INPUT_1);
        let value = calibration_sum_digit(&input);
        assert_eq!(value, 142);
    }

    #[test]
    fn test_calibration_sum_all() {
        let input = input_generator(EXAMPLE_INPUT_2);
        let value = calibration_sum_all(&input);
        assert_eq!(value, 281);
    }
}
