/*
    --- Day 11: Cosmic Expansion ---
    You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

    He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

    Maybe you can help him with the analysis to speed things up?

    The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

    Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

    In the above example, three columns and two rows contain no galaxies:

    v  v  v
    ...#......
    .......#..
    #.........
    >..........<
    ......#...
    .#........
    .........#
    >..........<
    .......#..
    #...#.....
    ^  ^  ^
    These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

    ....#........
    .........#...
    #............
    .............
    .............
    ........#....
    .#...........
    ............#
    .............
    .............
    .........#...
    #....#.......
    Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

    ....1........
    .........2...
    3............
    .............
    .............
    ........4....
    .5...........
    ............6
    .............
    .............
    .........7...
    8....9.......
    In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

    For example, here is one of the shortest paths between galaxies 5 and 9:

    ....1........
    .........2...
    3............
    .............
    .............
    ........4....
    .5...........
    .##.........6
    ..##.........
    ...##........
    ....##...7...
    8....9.......
    This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

    Between galaxy 1 and galaxy 7: 15
    Between galaxy 3 and galaxy 6: 17
    Between galaxy 8 and galaxy 9: 5
    In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

    Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

    --- Part Two ---
    The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

    Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

    (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

    Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
*/

use common::Point2;
use itertools::Itertools;

pub struct GalaxyMap {
    galaxies: Vec<Point2>,
    empty_rows: Vec<i32>,
    empty_cols: Vec<i32>,
}

impl GalaxyMap {
    fn distance(&self, a: Point2, b: Point2, factor: u64) -> u64 {
        let range = Point2::get_range(&[a, b]).unwrap();
        let e_rows = self
            .empty_rows
            .iter()
            .copied()
            .filter(|&y| range.contains(Point2 { x: a.x, y }))
            .count();
        let e_cols = self
            .empty_cols
            .iter()
            .copied()
            .filter(|&x| range.contains(Point2 { x, y: a.y }))
            .count();
        let expansion_offset = (factor - 1) * ((e_rows + e_cols) as u64);
        Point2::manhattan(a, b) as u64 + expansion_offset
    }

    fn all_distances(&self, factor: u64) -> u64 {
        self.galaxies
            .iter()
            .copied()
            .combinations(2)
            .map(|combo| self.distance(combo[0], combo[1], factor))
            .sum()
    }
}

impl From<&str> for GalaxyMap {
    fn from(value: &str) -> Self {
        let galaxies = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point2 {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();

        let empty_rows = value
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                if line.chars().all(|c| c != '#') {
                    Some(y as i32)
                } else {
                    None
                }
            })
            .collect();

        let mut lines = value.lines();
        let empty: Vec<bool> = lines.next().unwrap().chars().map(|c| c != '#').collect();
        let empty_cols = lines
            .fold(empty, |mut acc, line| {
                acc.iter_mut()
                    .zip(line.chars())
                    .for_each(|(e, c)| *e = *e && c != '#');
                acc
            })
            .into_iter()
            .enumerate()
            .filter_map(|(i, e)| if e { Some(i as i32) } else { None })
            .collect();

        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> GalaxyMap {
    input.into()
}

#[aoc(day11, part1)]
pub fn part1(input: &GalaxyMap) -> u64 {
    let value = input.all_distances(2);
    assert_eq!(value, 10422930);
    value
}

#[aoc(day11, part2)]
pub fn part2(input: &GalaxyMap) -> u64 {
    let value = input.all_distances(1_000_000);
    assert_eq!(value, 699909023130);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_all_distances() {
        let input = input_generator(EXAMPLE_INPUT);

        let value = input.all_distances(2);
        assert_eq!(value, 374);

        let value = input.all_distances(10);
        assert_eq!(value, 1030);

        let value = input.all_distances(100);
        assert_eq!(value, 8410);
    }
}
