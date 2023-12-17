/*
    --- Day 14: Parabolic Reflector Dish ---
    You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

    The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

    This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

    Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

    In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    Start by tilting the lever so all of the rocks will slide north as far as they will go:

    OOOO.#.O..
    OO..#....#
    OO..O##..O
    O..#.OO...
    ........#.
    ..#....#.#
    ..O..#.O.O
    ..O.......
    #....###..
    #....#....
    You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

    The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

    OOOO.#.O.. 10
    OO..#....#  9
    OO..O##..O  8
    O..#.OO...  7
    ........#.  6
    ..#....#.#  5
    ..O..#.O.O  4
    ..O.......  3
    #....###..  2
    #....#....  1
    The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

    Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?

    --- Part Two ---
    The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

    Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

    Here's what happens in the example above after each of the first few cycles:

    After 1 cycle:
    .....#....
    ....#...O#
    ...OO##...
    .OO#......
    .....OOO#.
    .O#...O#.#
    ....O#....
    ......OOOO
    #...O###..
    #..OO#....

    After 2 cycles:
    .....#....
    ....#...O#
    .....##...
    ..O#......
    .....OOO#.
    .O#...O#.#
    ....O#...O
    .......OOO
    #..OO###..
    #.OOO#...O

    After 3 cycles:
    .....#....
    ....#...O#
    .....##...
    ..O#......
    .....OOO#.
    .O#...O#.#
    ....O#...O
    .......OOO
    #...O###.O
    #.OOO#...O
    This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

    In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

    Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?
*/

use std::collections::HashMap;

use common::{Cardinal, Point2, TileChar, TileMap};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Rocks {
    Round,
    Cube,
}

impl TileChar for Rocks {
    fn to_char(&self) -> char {
        match *self {
            Rocks::Round => 'O',
            Rocks::Cube => '#',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            'O' => Some(Rocks::Round),
            '#' => Some(Rocks::Cube),
            _ => None,
        }
    }

    fn all_chars() -> Vec<char> {
        vec!['O', '#']
    }
}

#[derive(Clone)]
pub struct Platform {
    rocks: TileMap<Rocks>,
}

impl Platform {
    fn tilt(&mut self, dir: Cardinal) {
        let range = self.rocks.get_range().unwrap();

        match dir {
            Cardinal::North => {
                for x in range.x.0..=range.x.1 {
                    let mut write_p = Point2 { x, y: range.y.0 };
                    for y in range.y.0..=range.y.1 {
                        let p = Point2 { x, y };
                        match self.rocks.get(&p) {
                            Some(Rocks::Cube) => write_p.y = y + 1,
                            Some(Rocks::Round) => {
                                self.rocks.remove(&p);
                                self.rocks.insert(write_p, Rocks::Round);
                                write_p.y += 1;
                            }
                            None => (),
                        }
                    }
                }
            }
            Cardinal::South => {
                for x in range.x.0..=range.x.1 {
                    let mut write_p = Point2 { x, y: range.y.1 };
                    for y in (range.y.0..=range.y.1).rev() {
                        let p = Point2 { x, y };
                        match self.rocks.get(&p) {
                            Some(Rocks::Cube) => write_p.y = y - 1,
                            Some(Rocks::Round) => {
                                self.rocks.remove(&p);
                                self.rocks.insert(write_p, Rocks::Round);
                                write_p.y -= 1;
                            }
                            None => (),
                        }
                    }
                }
            }
            Cardinal::East => {
                for y in range.y.0..=range.y.1 {
                    let mut write_p = Point2 { x: range.x.1, y };
                    for x in (range.x.0..=range.x.1).rev() {
                        let p = Point2 { x, y };
                        match self.rocks.get(&p) {
                            Some(Rocks::Cube) => write_p.x = x - 1,
                            Some(Rocks::Round) => {
                                self.rocks.remove(&p);
                                self.rocks.insert(write_p, Rocks::Round);
                                write_p.x -= 1;
                            }
                            None => (),
                        }
                    }
                }
            }
            Cardinal::West => {
                for y in range.y.0..=range.y.1 {
                    let mut write_p = Point2 { x: range.x.0, y };
                    for x in range.x.0..=range.x.1 {
                        let p = Point2 { x, y };
                        match self.rocks.get(&p) {
                            Some(Rocks::Cube) => write_p.x = x + 1,
                            Some(Rocks::Round) => {
                                self.rocks.remove(&p);
                                self.rocks.insert(write_p, Rocks::Round);
                                write_p.x += 1;
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        for dir in [
            Cardinal::North,
            Cardinal::West,
            Cardinal::South,
            Cardinal::East,
        ] {
            self.tilt(dir);
        }
    }

    fn hash_token(&self) -> Vec<(Point2, Rocks)> {
        let mut vec: Vec<(Point2, Rocks)> = self.rocks.iter().map(|(p, r)| (*p, *r)).collect();
        vec.sort_by_key(|(p, _r)| *p);
        vec
    }

    fn spin_n(&mut self, n: usize) {
        let mut cache: HashMap<Vec<(Point2, Rocks)>, usize> = HashMap::new();

        let mut i = 0;
        while i < n {
            if let Some(prev_idx) = cache.insert(self.hash_token(), i) {
                let cycle = i - prev_idx;
                let remaining = n - i;
                let skips = remaining / cycle;
                i += skips * cycle;
                cache.clear(); // This just avoids trying to skip again
            }
            self.spin_cycle();
            i += 1;
        }
    }

    fn calc_load(&self) -> i32 {
        let mut load = 0;
        let range = self.rocks.get_range().unwrap();

        for x in range.x.0..=range.x.1 {
            for y in range.y.0..=range.y.1 {
                let p = Point2 { x, y };
                if matches!(self.rocks.get(&p), Some(Rocks::Round)) {
                    load += range.y.1 - y + 1;
                }
            }
        }

        load
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let rocks = TileMap::from_string(value);
        Self { rocks }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Platform {
    input.into()
}

#[aoc(day14, part1)]
pub fn part1(input: &Platform) -> i32 {
    let mut platform = input.clone();
    platform.tilt(Cardinal::North);
    let value = platform.calc_load();
    assert_eq!(value, 109098);
    value
}

#[aoc(day14, part2)]
pub fn part2(input: &Platform) -> i32 {
    let mut platform = input.clone();
    platform.spin_n(1_000_000_000);
    let value = platform.calc_load();
    assert_eq!(value, 100064);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_calc_load() {
        let mut platform = input_generator(EXAMPLE_INPUT);
        platform.tilt(Cardinal::North);
        let value = platform.calc_load();
        assert_eq!(value, 136);
    }

    #[test]
    fn test_spin_cycle() {
        let mut platform = input_generator(EXAMPLE_INPUT);
        platform.spin_n(1_000_000_000);
        let value = platform.calc_load();
        assert_eq!(value, 64);
    }
}
