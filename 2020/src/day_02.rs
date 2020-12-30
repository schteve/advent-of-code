/*
    --- Day 2: Password Philosophy ---
    Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

    The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

    Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

    To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

    For example, suppose you have the following list:

    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

    In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

    How many passwords are valid according to their policies?

    --- Part Two ---
    While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

    The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

    Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

    Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
    How many passwords are valid according to the new interpretation of the policies?
*/

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, satisfy, space1},
    combinator::{map, map_res},
    multi::many1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Policy {
    range: (u32, u32),
    letter: char,
}

impl Policy {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (range, letter)) = separated_pair(
            separated_pair(
                map_res(digit1, |min: &str| min.parse::<u32>()),
                char('-'),
                map_res(digit1, |max: &str| max.parse::<u32>()),
            ),
            space1,
            satisfy(|c| 'a' <= c && c <= 'z'),
        )(input)?;

        Ok((input, Self { range, letter }))
    }
}

#[derive(Debug)]
pub struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (policy, password)) = separated_pair(
            Policy::parser,
            tag(": "),
            map(many1(satisfy(|c| 'a' <= c && c <= 'z')), |p: Vec<char>| {
                p.into_iter().collect::<String>()
            }),
        )(input)?;

        Ok((input, Self { policy, password }))
    }

    fn is_password_valid1(&self) -> bool {
        let policy_letter_count = self
            .password
            .chars()
            .filter(|&c| c == self.policy.letter)
            .count() as u32;
        self.policy.range.0 <= policy_letter_count && policy_letter_count <= self.policy.range.1
    }

    fn is_password_valid2(&self) -> bool {
        let mut count = 0;
        if let Some(nth) = self
            .password
            .chars()
            .nth((self.policy.range.0 - 1) as usize)
        {
            if nth == self.policy.letter {
                count += 1;
            }
        }

        if let Some(nth) = self
            .password
            .chars()
            .nth((self.policy.range.1 - 1) as usize)
        {
            if nth == self.policy.letter {
                count += 1;
            }
        }

        count == 1
    }
}

fn count_valid_passwords1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(Entry::is_password_valid1)
        .filter(|&b| b == true)
        .count()
}

fn count_valid_passwords2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(Entry::is_password_valid2)
        .filter(|&b| b == true)
        .count()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input.lines().map(Entry::from_string).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> usize {
    let valid_count = count_valid_passwords1(input);
    assert_eq!(valid_count, 456);
    valid_count
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> usize {
    let valid_count = count_valid_passwords2(input);
    assert_eq!(valid_count, 308);
    valid_count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_is_password_valid_1() {
        let entries = input_generator(EXAMPLE_INPUT);
        let valid: Vec<bool> = entries.iter().map(Entry::is_password_valid1).collect();
        assert_eq!(valid, [true, false, true]);
    }

    #[test]
    fn test_count_valid_passwords1() {
        let entries = input_generator(EXAMPLE_INPUT);
        let valid_count = count_valid_passwords1(&entries);
        assert_eq!(valid_count, 2);
    }

    #[test]
    fn test_is_password_valid_2() {
        let entries = input_generator(EXAMPLE_INPUT);
        let valid: Vec<bool> = entries.iter().map(Entry::is_password_valid2).collect();
        assert_eq!(valid, [true, false, false]);
    }

    #[test]
    fn test_count_valid_passwords2() {
        let entries = input_generator(EXAMPLE_INPUT);
        let valid_count = count_valid_passwords2(&entries);
        assert_eq!(valid_count, 1);
    }
}
