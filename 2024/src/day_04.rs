/*
    --- Day 4: Ceres Search ---
    "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

    As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

    This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


    ..X...
    .SAMX.
    .A..A.
    XMAS.S
    .X....
    The actual word search will be full of letters instead. For example:

    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

    ....XXMAS.
    .SAMXMS...
    ...S..A...
    ..A.A.MS.X
    XMASAMX.MM
    X.....XA.A
    S.S.S.S.SS
    .A.A.A.A.A
    ..M.M.M.MM
    .X.X.XMASX
    Take a look at the little Elf's word search. How many times does XMAS appear?

    --- Part Two ---
    The Elf looks quizzically at you. Did you misunderstand the assignment?

    Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

    M.S
    .A.
    M.S
    Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

    Here's the same example from before, but this time all of the X-MASes have been kept instead:

    .M.S......
    ..A..MSMS.
    .M.S.MAA..
    ..A.ASMSM.
    .M.S.M....
    ..........
    S.S.S.S.S.
    .A.A.A.A..
    M.M.M.M.M.
    ..........
    In this example, an X-MAS appears 9 times.

    Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?
*/

use std::collections::HashMap;

use common::Point2;

enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

pub struct WordSearch {
    letters: HashMap<Point2, char>,
}

impl WordSearch {
    fn parse(input: &str) -> Self {
        let mut letters = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = (x as i32, y as i32).into();
                letters.insert(p, c);
            }
        }
        Self { letters }
    }

    fn get_4_letters(&self, start: Point2, dir: Direction) -> Option<[char; 4]> {
        let mut output = ['-'; 4];

        let mut p = start;
        for out in &mut output {
            if let Some(c) = self.letters.get(&p) {
                *out = *c;
            } else {
                return None;
            }

            p += match dir {
                Direction::UpLeft => (-1, -1),
                Direction::Up => (0, -1),
                Direction::UpRight => (1, -1),
                Direction::Right => (1, 0),
                Direction::DownRight => (1, 1),
                Direction::Down => (0, 1),
                Direction::DownLeft => (-1, 1),
                Direction::Left => (-1, 0),
            }
        }

        Some(output)
    }

    fn count_xmas(&self) -> u64 {
        let mut count = 0;

        let range = Point2::get_range(self.letters.keys()).unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let p = Point2 { x, y };
                if self.letters.get(&p) == Some(&'X') {
                    for dir in [
                        Direction::UpLeft,
                        Direction::Up,
                        Direction::UpRight,
                        Direction::Right,
                        Direction::DownRight,
                        Direction::Down,
                        Direction::DownLeft,
                        Direction::Left,
                    ] {
                        if self.get_4_letters(p, dir) == Some(['X', 'M', 'A', 'S']) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn get_x_letters(&self, start: Point2) -> Option<[char; 4]> {
        let mut output = ['-'; 4];

        let points = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
        for (p, out) in points.iter().zip(output.iter_mut()) {
            if let Some(letter) = self.letters.get(&(start + p)) {
                *out = *letter;
            } else {
                return None;
            }
        }

        Some(output)
    }

    fn count_x_mas(&self) -> u64 {
        let mut count = 0;

        let range = Point2::get_range(self.letters.keys()).unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let p = Point2 { x, y };
                if self.letters.get(&p) == Some(&'A') {
                    if let Some(x) = self.get_x_letters(p) {
                        if x == ['M', 'M', 'S', 'S']
                            || x == ['M', 'S', 'S', 'M']
                            || x == ['S', 'S', 'M', 'M']
                            || x == ['S', 'M', 'M', 'S']
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> WordSearch {
    WordSearch::parse(input)
}

#[aoc(day4, part1)]
pub fn part1(input: &WordSearch) -> u64 {
    let value = input.count_xmas();
    assert_eq!(value, 2493);
    value
}

#[aoc(day4, part2)]
pub fn part2(input: &WordSearch) -> u64 {
    let value = input.count_x_mas();
    assert_eq!(value, 1890);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_count_xmas() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.count_xmas();
        assert_eq!(value, 18);
    }

    #[test]
    fn test_count_x_mas() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.count_x_mas();
        assert_eq!(value, 9);
    }
}
