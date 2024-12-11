/*
    --- Day 8: Resonant Collinearity ---
    You find yourselves on the roof of a top-secret Easter Bunny installation.

    While The Historians do their thing, you take a look at the familiar huge antenna. Much to your surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!

    Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map (your puzzle input) of these antennas. For example:

    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas. In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other. This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

    So, for these two antennas with frequency a, they create the two antinodes marked with #:

    ..........
    ...#......
    ..........
    ....a.....
    ..........
    .....a....
    ..........
    ......#...
    ..........
    ..........
    Adding a third antenna with the same frequency creates several more antinodes. It would ideally add four antinodes, but two are off the right side of the map, so instead it adds only two:

    ..........
    ...#......
    #.........
    ....a.....
    ........a.
    .....a....
    ..#.......
    ......#...
    ..........
    ..........
    Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its location:

    ..........
    ...#......
    #.........
    ....a.....
    ........a.
    .....a....
    ..#.......
    ......A...
    ..........
    ..........
    The first example has antennas with two different frequencies, so the antinodes they create look like this, plus an antinode overlapping the topmost A-frequency antenna:

    ......#....#
    ...#....0...
    ....#0....#.
    ..#....0....
    ....0....#..
    .#....A.....
    ...#........
    #......#....
    ........A...
    .........A..
    ..........#.
    ..........#.
    Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations that contain an antinode within the bounds of the map.

    Calculate the impact of the signal. How many unique locations within the bounds of the map contain an antinode?

    --- Part Two ---
    Watching over your shoulder as you work, one of The Historians asks if you took the effects of resonant harmonics into your calculations.

    Whoops!

    After updating your model, it turns out that an antinode occurs at any grid position exactly in line with at least two antennas of the same frequency, regardless of distance. This means that some of the new antinodes will occur at the position of each antenna (unless that antenna is the only one of its frequency).

    So, these three T-frequency antennas now create many antinodes:

    T....#....
    ...T......
    .T....#...
    .........#
    ..#.......
    ..........
    ...#......
    ..........
    ....#.....
    ..........
    In fact, the three T-frequency antennas are all exactly in line with two antennas, so they are all also antinodes! This brings the total number of antinodes in the above example to 9.

    The original example now has 34 antinodes, including the antinodes that appear on every antenna:

    ##....#....#
    .#.#....0...
    ..#.#0....#.
    ..##...0....
    ....0....#..
    .#...#A....#
    ...#..#.....
    #....#.#....
    ..#.....A...
    ....#....A..
    .#........#.
    ...#......##
    Calculate the impact of the signal using this updated model. How many unique locations within the bounds of the map contain an antinode?
*/

use std::collections::{HashMap, HashSet};

use common::{Point2, Range2};

pub struct CityMap {
    antennas: HashMap<char, Vec<Point2>>,
    range: Range2,
}

impl CityMap {
    fn parse(input: &str) -> Self {
        let mut antennas = HashMap::new();

        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let p = Point2 {
                        x: x as i32,
                        y: y as i32,
                    };
                    let entry = antennas.entry(c).or_insert_with(Vec::new);
                    entry.push(p);
                }
                max_x = std::cmp::max(max_x, x as i32);
            }
            max_y = std::cmp::max(max_y, y as i32);
        }

        let range = Range2 {
            x: (0, max_x),
            y: (0, max_y),
        };

        Self { antennas, range }
    }

    fn count_antinodes(&self) -> u64 {
        let mut antinodes = HashSet::new();

        for (_c, antennas) in self.antennas.iter() {
            for i in 0..antennas.len() {
                for j in 0..antennas.len() {
                    if i != j {
                        let diff = antennas[j] - antennas[i];
                        let antinode = antennas[i] - diff;
                        if self.range.contains(antinode) {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }

        antinodes.len() as u64
    }

    fn count_antinodes_harmonic(&self) -> u64 {
        let mut antinodes = HashSet::new();

        for (_c, antennas) in self.antennas.iter() {
            for i in 0..antennas.len() {
                for j in 0..antennas.len() {
                    if i != j {
                        let diff = antennas[j] - antennas[i];
                        let mut antinode = antennas[i];
                        while self.range.contains(antinode) {
                            antinodes.insert(antinode);
                            antinode -= diff;
                        }
                    }
                }
            }
        }

        antinodes.len() as u64
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> CityMap {
    CityMap::parse(input)
}

#[aoc(day8, part1)]
pub fn part1(input: &CityMap) -> u64 {
    let value = input.count_antinodes();
    assert_eq!(value, 249);
    value
}

#[aoc(day8, part2)]
pub fn part2(input: &CityMap) -> u64 {
    let value = input.count_antinodes_harmonic();
    assert_eq!(value, 905);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_count_antinodes() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.count_antinodes();
        assert_eq!(value, 14);
    }

    #[test]
    fn test_count_antinodes_harmonic() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.count_antinodes_harmonic();
        assert_eq!(value, 34);
    }
}
