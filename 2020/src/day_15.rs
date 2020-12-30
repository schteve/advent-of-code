/*
    --- Day 15: Rambunctious Recitation ---
    You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.

    While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!

    In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:

    If that was the first time the number has been spoken, the current player says 0.
    Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
    So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).

    For example, suppose the starting numbers are 0,3,6:

    Turn 1: The 1st number spoken is a starting number, 0.
    Turn 2: The 2nd number spoken is a starting number, 3.
    Turn 3: The 3rd number spoken is a starting number, 6.
    Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
    Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
    Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
    Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
    Turn 8: Since 1 is new, the 8th number spoken is 0.
    Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
    Turn 10: 4 is new, so the 10th number spoken is 0.
    (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)

    Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.

    Here are a few more examples:

    Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
    Given your starting numbers, what will be the 2020th number spoken?

    --- Part Two ---
    Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:

    Given 0,3,6, the 30000000th number spoken is 175594.
    Given 1,3,2, the 30000000th number spoken is 2578.
    Given 2,1,3, the 30000000th number spoken is 3544142.
    Given 1,2,3, the 30000000th number spoken is 261214.
    Given 2,3,1, the 30000000th number spoken is 6895259.
    Given 3,2,1, the 30000000th number spoken is 18.
    Given 3,1,2, the 30000000th number spoken is 362.
    Given your starting numbers, what will be the 30000000th number spoken?
*/

use std::collections::HashMap;

struct MemoryGame {
    start: Vec<usize>,
    spoken: HashMap<usize, usize>,
    next_idx: usize,
    next_value: usize,
}

impl MemoryGame {
    fn from_slice(input: &[usize]) -> Self {
        Self {
            start: input.to_vec(),
            spoken: HashMap::new(),
            next_idx: 0,
            next_value: input[0],
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let tmp = self.next_value;
        let prev = self.spoken.insert(self.next_value, self.next_idx);
        if self.spoken.len() < self.start.len() {
            self.next_value = self.start[self.spoken.len()];
        } else {
            match prev {
                Some(prev_idx) => self.next_value = self.next_idx - prev_idx,
                None => self.next_value = 0,
            }
        }
        self.next_idx += 1;
        Some(tmp)
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut game = MemoryGame::from_slice(input);
    let spoken_2020 = game.nth(2020 - 1).unwrap();
    assert_eq!(spoken_2020, 319);
    spoken_2020
}

#[aoc(day15, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut game = MemoryGame::from_slice(input);
    let spoken_30000000 = game.nth(30000000 - 1).unwrap();
    assert_eq!(spoken_30000000, 2424);
    spoken_30000000
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT_EXAMPLE1: &str = "0,3,6";
    static INPUT_EXAMPLE2: &str = "1,3,2";
    static INPUT_EXAMPLE3: &str = "2,1,3";
    static INPUT_EXAMPLE4: &str = "1,2,3";
    static INPUT_EXAMPLE5: &str = "2,3,1";
    static INPUT_EXAMPLE6: &str = "3,2,1";
    static INPUT_EXAMPLE7: &str = "3,1,2";

    #[test]
    #[ignore]
    fn test_next() {
        let start = input_generator(INPUT_EXAMPLE1);
        let game = MemoryGame::from_slice(&start);
        assert_eq!(
            game.take(10).collect::<Vec<usize>>(),
            [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]
        );

        let start = input_generator(INPUT_EXAMPLE1);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(436));

        let start = input_generator(INPUT_EXAMPLE2);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(1));

        let start = input_generator(INPUT_EXAMPLE3);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(10));

        let start = input_generator(INPUT_EXAMPLE4);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(27));

        let start = input_generator(INPUT_EXAMPLE5);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(78));

        let start = input_generator(INPUT_EXAMPLE6);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(438));

        let start = input_generator(INPUT_EXAMPLE7);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(2020 - 1), Some(1836));

        let start = input_generator(INPUT_EXAMPLE1);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(175594));

        let start = input_generator(INPUT_EXAMPLE2);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(2578));

        let start = input_generator(INPUT_EXAMPLE3);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(3544142));

        let start = input_generator(INPUT_EXAMPLE4);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(261214));

        let start = input_generator(INPUT_EXAMPLE5);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(6895259));

        let start = input_generator(INPUT_EXAMPLE6);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(18));

        let start = input_generator(INPUT_EXAMPLE7);
        let mut game = MemoryGame::from_slice(&start);
        assert_eq!(game.nth(30000000 - 1), Some(362));
    }
}
