/*
    --- Day 23: Crab Cups ---
    The small crab challenges you to a game! The crab is going to mix up some cups, and you have to predict where they'll end up.

    The cups will be arranged in a circle and labeled clockwise (your puzzle input). For example, if your labeling were 32415, there would be five cups in the circle; going clockwise around the circle from the first cup, the cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.

    Before the crab starts, it will designate the first cup in your list as the current cup. The crab is then going to do 100 moves.

    Each move, the crab does the following actions:

    The crab picks up the three cups that are immediately clockwise of the current cup. They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
    The crab selects a destination cup: the cup with a label equal to the current cup's label minus one. If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up. If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
    The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
    The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
    For example, suppose your cup labeling were 389125467. If the crab were to do merely 10 moves, the following changes would occur:

    -- move 1 --
    cups: (3) 8  9  1  2  5  4  6  7
    pick up: 8, 9, 1
    destination: 2

    -- move 2 --
    cups:  3 (2) 8  9  1  5  4  6  7
    pick up: 8, 9, 1
    destination: 7

    -- move 3 --
    cups:  3  2 (5) 4  6  7  8  9  1
    pick up: 4, 6, 7
    destination: 3

    -- move 4 --
    cups:  7  2  5 (8) 9  1  3  4  6
    pick up: 9, 1, 3
    destination: 7

    -- move 5 --
    cups:  3  2  5  8 (4) 6  7  9  1
    pick up: 6, 7, 9
    destination: 3

    -- move 6 --
    cups:  9  2  5  8  4 (1) 3  6  7
    pick up: 3, 6, 7
    destination: 9

    -- move 7 --
    cups:  7  2  5  8  4  1 (9) 3  6
    pick up: 3, 6, 7
    destination: 8

    -- move 8 --
    cups:  8  3  6  7  4  1  9 (2) 5
    pick up: 5, 8, 3
    destination: 1

    -- move 9 --
    cups:  7  4  1  5  8  3  9  2 (6)
    pick up: 7, 4, 1
    destination: 5

    -- move 10 --
    cups: (5) 7  4  1  8  3  9  2  6
    pick up: 7, 4, 1
    destination: 3

    -- final --
    cups:  5 (8) 3  7  4  1  9  2  6
    In the above example, the cups' values are the labels as they appear moving clockwise around the circle; the current cup is marked with ( ).

    After the crab is done, what order will the cups be in? Starting after the cup labeled 1, collect the other cups' labels clockwise into a single string with no extra characters; each number except 1 should appear exactly once. In the above example, after 10 moves, the cups clockwise from 1 are labeled 9, 2, 6, 5, and so on, producing 92658374. If the crab were to complete all 100 moves, the order after cup 1 would be 67384529.

    Using your labeling, simulate 100 moves. What are the labels on the cups after cup 1?

    --- Part Two ---
    Due to what you can only assume is a mistranslation (you're not exactly fluent in Crab), you are quite surprised when the crab starts arranging many cups in a circle on your raft - one million (1000000) in total.

    Your labeling is still correct for the first few cups; after that, the remaining cups are just numbered in an increasing fashion starting from the number after the highest number in your list and proceeding one by one until one million is reached. (For example, if your labeling were 54321, the cups would be numbered 5, 4, 3, 2, 1, and then start counting up from 6 until one million is reached.) In this way, every number from one through one million is used exactly once.

    After discovering where you made the mistake in translating Crab Numbers, you realize the small crab isn't going to do merely 100 moves; the crab is going to do ten million (10000000) moves!

    The crab is going to hide your stars - one each - under the two cups that will end up immediately clockwise of cup 1. You can have them if you predict what the labels on those cups will be when the crab is finished.

    In the above example (389125467), this would be 934001 and then 159792; multiplying these together produces 149245887792.

    Determine which two cups will end up immediately clockwise of cup 1. What do you get if you multiply their labels together?
*/

use nom::{character::complete::one_of, combinator::map, multi::many1, IResult};
use std::fmt::Write;

fn u32_list_parser(input: &str) -> IResult<&str, Vec<u32>> {
    many1(map(one_of("123456789"), |c| c.to_digit(10).unwrap()))(input)
}

#[derive(Clone)]
pub struct CrabCups {
    cups: Vec<u32>, // A singly-linked list; 0th slot is empty so we can 1-index
    current_cup: u32,
    max_cup: u32,
}

impl CrabCups {
    fn from_list(input: &[u32], max_cup: u32) -> Self {
        let mut cups = vec![0; (max_cup + 1) as usize];
        let mut cups_list_iter = input.iter();
        let current_cup = *cups_list_iter.next().unwrap();
        let mut last_cup = current_cup;
        for cup in cups_list_iter {
            cups[last_cup as usize] = *cup;
            last_cup = *cup;
        }
        for cup in input.len() as u32 + 1..=max_cup {
            cups[last_cup as usize] = cup;
            last_cup = cup;
        }
        cups[last_cup as usize] = current_cup;

        Self {
            cups,
            current_cup,
            max_cup,
        }
    }

    fn insert3(&mut self, after: u32, cups: [u32; 3]) {
        assert!(after >= 1 && after <= self.max_cup);
        assert!(self.cups[after as usize] != 0);

        let [a, b, c] = cups;
        assert!(a >= 1 && a <= self.max_cup);
        assert!(b >= 1 && b <= self.max_cup);
        assert!(c >= 1 && c <= self.max_cup);

        let next = self.cups[after as usize];
        self.cups[after as usize] = a;
        self.cups[a as usize] = b;
        self.cups[b as usize] = c;
        self.cups[c as usize] = next;
    }

    fn remove3(&mut self, after: u32) -> [u32; 3] {
        assert!(after >= 1 && after <= self.max_cup);
        assert!(self.cups[after as usize] != 0);

        let a = self.cups[after as usize];
        let b = self.cups[a as usize];
        let c = self.cups[b as usize];

        self.cups[after as usize] = self.cups[c as usize];
        self.cups[a as usize] = 0;
        self.cups[b as usize] = 0;
        self.cups[c as usize] = 0;
        [a, b, c]
    }

    fn make_move(&mut self, num: u32) {
        for _ in 0..num {
            // Pick up 3 cups immediately clockwise of the current cup
            let [a, b, c] = self.remove3(self.current_cup);

            // Select destination cup
            let mut dest_cup = self.current_cup;
            loop {
                dest_cup -= 1;
                if dest_cup == 0 {
                    dest_cup = self.max_cup;
                }
                if a != dest_cup && b != dest_cup && c != dest_cup {
                    break;
                }
            }

            // Place 3 cups immediately clockwise of the destination cup
            self.insert3(dest_cup, [a, b, c]);

            // Select new current cup, immediately clockwise of current cup
            self.current_cup = self.cups[self.current_cup as usize];
        }
    }

    fn order(&self, start: u32, num: u32) -> String {
        assert_ne!(start, 0);

        let mut output = String::new();
        let mut cup = start;
        for _ in 0..num {
            write!(output, "{}", cup).unwrap();
            cup = self.cups[cup as usize];
        }
        output
    }
}

impl std::fmt::Display for CrabCups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.order(self.current_cup, 9))
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u32> {
    u32_list_parser(input).unwrap().1
}

#[aoc(day23, part1)]
pub fn part1(input: &[u32]) -> String {
    let mut crab_cups = CrabCups::from_list(input, 9);
    crab_cups.make_move(100);
    let labels = crab_cups.order(crab_cups.cups[1], 8);
    assert_eq!(labels, "82573496");
    labels
}

#[aoc(day23, part2)]
pub fn part2(input: &[u32]) -> u64 {
    let mut crab_cups = CrabCups::from_list(input, 1000000);
    crab_cups.make_move(10000000);
    let a = crab_cups.cups[1];
    let b = crab_cups.cups[a as usize];
    let label_product = a as u64 * b as u64;
    assert_eq!(label_product, 11498506800);
    label_product
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "389125467";

    #[test]
    fn test_from_list() {
        let input = input_generator(EXAMPLE_INPUT);

        let crab_cups = CrabCups::from_list(&input, 9);
        assert_eq!(crab_cups.cups, [0, 2, 5, 8, 6, 4, 7, 3, 9, 1]);

        let crab_cups = CrabCups::from_list(&input, 20);
        assert_eq!(
            crab_cups.cups,
            [0, 2, 5, 8, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3]
        );
    }

    #[test]
    fn test_make_move() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut crab_cups = CrabCups::from_list(&input, 9);
        assert_eq!(crab_cups.to_string(), "389125467");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "289154673");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "546789132");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "891346725");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "467913258");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "136792584");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "936725841");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "258367419");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "674158392");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "574183926");

        crab_cups.make_move(1);
        assert_eq!(crab_cups.to_string(), "837419265");
    }

    #[test]
    #[ignore]
    fn test_cups_order() {
        let input = input_generator(EXAMPLE_INPUT);

        let mut crab_cups = CrabCups::from_list(&input, 9);
        crab_cups.make_move(10);
        assert_eq!(crab_cups.order(crab_cups.cups[1], 8), "92658374");

        let mut crab_cups = CrabCups::from_list(&input, 9);
        crab_cups.make_move(100);
        assert_eq!(crab_cups.order(crab_cups.cups[1], 8), "67384529");

        let mut crab_cups = CrabCups::from_list(&input, 1000000);
        crab_cups.make_move(10000000);
        let a = crab_cups.cups[1];
        let b = crab_cups.cups[a as usize];
        let label_product = a as u64 * b as u64;
        assert_eq!(label_product, 149245887792);
    }
}
