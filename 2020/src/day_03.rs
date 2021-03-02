/*
    --- Day 3: Toboggan Trajectory ---
    With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

    Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#
    These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:

    ..##.........##.........##.........##.........##.........##.......  --->
    #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
    .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
    ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
    .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
    ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
    .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
    .#........#.#........#.#........#.#........#.#........#.#........#
    #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
    #...##....##...##....##...##....##...##....##...##....##...##....#
    .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
    You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

    The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

    From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

    The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

    ..##.........##.........##.........##.........##.........##.......  --->
    #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
    .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
    ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
    .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
    ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
    .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
    .#........#.#........X.#........#.#........#.#........#.#........#
    #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
    #...##....##...##....##...#X....##...##....##...##....##...##....#
    .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
    In this example, traversing the map using this slope would cause you to encounter 7 trees.

    Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?

    --- Part Two ---
    Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

    Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

    What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/

use crate::common::{modulo, Point, TileSet};

pub struct Map {
    tileset: TileSet,
    x_max: i32,
    y_max: i32,
}

impl Map {
    fn from_string(input: &str) -> Self {
        let tileset = TileSet::from_string(input, '#');
        let range = tileset.get_range().unwrap();

        Self {
            tileset,
            x_max: range.0 .1,
            y_max: range.1 .1,
        }
    }

    fn ride_toboggan(&self, x_delta: i32, y_delta: i32) -> usize {
        let mut p = Point::origin();
        let mut tree_count = 0;

        while p.y <= self.y_max {
            if self.tileset.contains(&p) {
                tree_count += 1;
            }

            p.x = modulo(p.x + x_delta, self.x_max + 1);
            p.y += y_delta;
        }

        tree_count
    }

    fn ride_toboggan_many(&self) -> usize {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        slopes
            .iter()
            .map(|&(x, y)| self.ride_toboggan(x, y))
            .product()
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[aoc(day3, part1)]
pub fn part1(input: &Map) -> usize {
    let tree_count = input.ride_toboggan(3, 1);
    assert_eq!(tree_count, 207);
    tree_count
}

#[aoc(day3, part2)]
pub fn part2(input: &Map) -> usize {
    let tree_product = input.ride_toboggan_many();
    assert_eq!(tree_product, 2655892800);
    tree_product
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_ride_toboggan() {
        let map = Map::from_string(EXAMPLE_INPUT);
        let tree_count = map.ride_toboggan(3, 1);
        assert_eq!(tree_count, 7);
    }

    #[test]
    fn test_ride_toboggan_many() {
        let map = Map::from_string(EXAMPLE_INPUT);

        let tree_count = map.ride_toboggan(1, 1);
        assert_eq!(tree_count, 2);

        let tree_count = map.ride_toboggan(3, 1);
        assert_eq!(tree_count, 7);

        let tree_count = map.ride_toboggan(5, 1);
        assert_eq!(tree_count, 3);

        let tree_count = map.ride_toboggan(7, 1);
        assert_eq!(tree_count, 4);

        let tree_count = map.ride_toboggan(1, 2);
        assert_eq!(tree_count, 2);

        let tree_product = map.ride_toboggan_many();
        assert_eq!(tree_product, 336);
    }
}
