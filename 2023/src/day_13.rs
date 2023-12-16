/*
    --- Day 13: Point of Incidence ---
    With your help, the hot springs team locates an appropriate spring which launches you neatly and precisely up to the edge of Lava Island.

    There's just one problem: you don't see any lava.

    You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered around. After a while, you make your way to a nearby cluster of mountains only to discover that the valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a consistent way; perhaps you should head in that direction?

    As you move through the valley of mirrors, you find that several of them have fallen from the large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.

    You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

    For example:

    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns.

    In the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

    123456789
        ><
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
        ><
    123456789
    In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1) has nowhere to reflect onto and can be ignored; every other column has a reflected column within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.

    The second pattern reflects across a horizontal line instead:

    1 #...##..# 1
    2 #....#..# 2
    3 ..##..### 3
    4v#####.##.v4
    5^#####.##.^5
    6 ..##..### 6
    7 #....#..# 7
    This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

    To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

    Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?

    --- Part Two ---
    You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing.

    Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.

    In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

    Here's the above example again:

    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    The first pattern's smudge is in the top-left corner. If the top-left # were instead ., it would have a different, horizontal line of reflection:

    1 ..##..##. 1
    2 ..#.##.#. 2
    3v##......#v3
    4^##......#^4
    5 ..#.##.#. 5
    6 ..##..##. 6
    7 #.#.##.#. 7
    With the smudge in the top-left corner repaired, a new horizontal line of reflection between rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.

    In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from . to #:

    1v#...##..#v1
    2^#...##..#^2
    3 ..##..### 3
    4 #####.##. 4
    5 #####.##. 5
    6 ..##..### 6
    7 #....#..# 7
    Now, the pattern has a different horizontal line of reflection between rows 1 and 2.

    Summarize your notes as before, but instead use the new different reflection lines. In this example, the first pattern's new horizontal line has 3 rows above it and the second pattern's new horizontal line has 1 row above it, summarizing to the value 400.

    In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?
*/

pub struct Mirror {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Mirror {
    fn find_reflection(&self, allow_smudge: bool) -> (Option<usize>, Option<usize>) {
        // Rows
        for reflection in 1..self.rows.len() {
            // BEFORE this row
            let mut smudged = false;
            let (left, right) = self.rows.split_at(reflection);
            if right.iter().zip(left.iter().rev()).all(|(a, b)| {
                let bit_diff = a ^ b;
                if bit_diff == 0 {
                    true
                } else if allow_smudge && bit_diff.is_power_of_two() && !smudged {
                    smudged = true;
                    true
                } else {
                    false
                }
            }) && (!allow_smudge || smudged)
            {
                return (Some(reflection), None);
            }
        }

        // Cols
        for reflection in 1..self.cols.len() {
            // BEFORE this col
            let mut smudged = false;
            let (left, right) = self.cols.split_at(reflection);
            if right.iter().zip(left.iter().rev()).all(|(a, b)| {
                let bit_diff = a ^ b;
                if bit_diff == 0 {
                    true
                } else if allow_smudge && bit_diff.is_power_of_two() && !smudged {
                    smudged = true;
                    true
                } else {
                    false
                }
            }) && (!allow_smudge || smudged)
            {
                return (None, Some(reflection));
            }
        }

        println!("{self}");
        panic!("No symmetry");
    }
}

impl From<&str> for Mirror {
    fn from(value: &str) -> Self {
        let rows: Vec<u32> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c == '#')
                    .enumerate()
                    .fold(0, |acc, (i, b)| acc | ((b as u32) << i))
            })
            .collect();

        let width = value.lines().next().unwrap().len();
        let cols = (0..width)
            .map(|i| {
                rows.iter()
                    .map(|row| (row & (1 << i)) != 0)
                    .enumerate()
                    .fold(0, |acc, (i, b)| acc | ((b as u32) << i))
            })
            .collect();

        Self { rows, cols }
    }
}

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for i in 0..self.cols.len() {
                let c = if (row & (1 << i)) != 0 { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn score_reflections(mirrors: &[Mirror], allow_smudge: bool) -> u64 {
    mirrors
        .iter()
        .map(|m| {
            let (horiz, vert) = m.find_reflection(allow_smudge);
            if let Some(h) = horiz {
                (h * 100) as u64
            } else if let Some(v) = vert {
                v as u64
            } else {
                unreachable!("No symmetry");
            }
        })
        .sum()
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Mirror> {
    input.split("\n\n").map(|chunk| chunk.into()).collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Mirror]) -> u64 {
    let value = score_reflections(input, false);
    assert_eq!(value, 30575);
    value
}

#[aoc(day13, part2)]
pub fn part2(input: &[Mirror]) -> u64 {
    let value = score_reflections(input, true);
    assert_eq!(value, 37478);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_reflections() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = score_reflections(&input, false);
        assert_eq!(value, 405);

        let value = score_reflections(&input, true);
        assert_eq!(value, 400);
    }
}
