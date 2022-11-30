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

// Use a sentinel value to indicate that the value has never been spoken before. This is
// this is significantly faster than using Option<32>, presumably because the compiler doesn't
// know that we only need values up to 30 million and so uses more than 32 bits for the option,
// leading to higher memory usage and poorer cache performance.
const NOT_SPOKEN: u32 = u32::MAX;

struct MemoryGame {
    start: Vec<usize>,
    spoken: Vec<u32>,
    next_idx: usize,
    next_value: usize,
}

impl MemoryGame {
    fn from_slice(input: &[usize], max_rounds: usize) -> Self {
        assert!(input.is_empty() == false);
        assert!(max_rounds < u32::MAX as usize);

        Self {
            start: input.to_vec(),
            spoken: vec![NOT_SPOKEN; max_rounds],
            next_idx: 0,
            next_value: input[0],
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.next_idx < self.spoken.len() {
            let curr_value = self.next_value;
            let curr_idx = self.next_idx;
            let prev = self.spoken[curr_value];
            self.spoken[curr_value] = curr_idx as u32;

            self.next_idx += 1;
            self.next_value = if self.next_idx < self.start.len() {
                self.start[self.next_idx]
            } else if prev == NOT_SPOKEN {
                0
            } else {
                curr_idx - prev as usize
            };
            Some(curr_value)
        } else {
            None
        }
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
    let game = MemoryGame::from_slice(input, 2020);
    let spoken = game.last().unwrap();
    assert_eq!(spoken, 319);
    spoken
}

#[aoc(day15, part2)]
pub fn part2(input: &[usize]) -> usize {
    let game = MemoryGame::from_slice(input, 30_000_000);
    let spoken = game.last().unwrap();
    assert_eq!(spoken, 2424);
    spoken
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
    fn test_next() {
        let start = input_generator(INPUT_EXAMPLE1);
        let game = MemoryGame::from_slice(&start, 10);
        assert_eq!(
            game.take(10).collect::<Vec<usize>>(),
            [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]
        );

        let start = input_generator(INPUT_EXAMPLE1);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(436));

        let start = input_generator(INPUT_EXAMPLE2);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(1));

        let start = input_generator(INPUT_EXAMPLE3);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(10));

        let start = input_generator(INPUT_EXAMPLE4);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(27));

        let start = input_generator(INPUT_EXAMPLE5);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(78));

        let start = input_generator(INPUT_EXAMPLE6);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(438));

        let start = input_generator(INPUT_EXAMPLE7);
        let game = MemoryGame::from_slice(&start, 2020);
        assert_eq!(game.last(), Some(1836));
    }

    #[test]
    #[ignore]
    fn test_next_long() {
        let start = input_generator(INPUT_EXAMPLE1);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(175594));

        let start = input_generator(INPUT_EXAMPLE2);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(2578));

        let start = input_generator(INPUT_EXAMPLE3);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(3544142));

        let start = input_generator(INPUT_EXAMPLE4);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(261214));

        let start = input_generator(INPUT_EXAMPLE5);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(6895259));

        let start = input_generator(INPUT_EXAMPLE6);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(18));

        let start = input_generator(INPUT_EXAMPLE7);
        let game = MemoryGame::from_slice(&start, 30_000_000);
        assert_eq!(game.last(), Some(362));
    }
}
