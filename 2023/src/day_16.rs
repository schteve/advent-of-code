/*
    --- Day 16: The Floor Will Be Lava ---
    With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

    Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

    Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

    The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

    You note the layout of the contraption (your puzzle input). For example:

    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

    If the beam encounters empty space (.), it continues in the same direction.
    If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
    If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
    If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
    Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

    In the above example, here is how the beam of light bounces around the contraption:

    >|<<<\....
    |v-.\^....
    .v...|->>>
    .v...v^.|.
    .v...v^...
    .v...v^..\
    .v../2\\..
    <->-/vv|..
    .|<<<2-|.\
    .v//.|.v..
    Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

    ######....
    .#...#....
    .#...#####
    .#...##...
    .#...##...
    .#...##...
    .#..####..
    ########..
    .#######..
    .#...#.#..
    Ultimately, in this example, 46 tiles become energized.

    The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?

    --- Part Two ---
    As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

    So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

    In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

    .|<2<\....
    |v-v\^....
    .v.v.|->>>
    .v.v.v^.|.
    .v.v.v^...
    .v.v.v^..\
    .v.v/2\\..
    <-2-/vv|..
    .|<<<2-|.\
    .v//.|.v..
    Using this configuration, 51 tiles are energized:

    .#####....
    .#.#.#....
    .#.#.#####
    .#.#.##...
    .#.#.##...
    .#.#.##...
    .#.#####..
    ########..
    .#######..
    .#...#.#..
    Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?
*/

use std::collections::HashSet;

use common::{Cardinal, Point2, TileChar, TileMap};

#[derive(Debug)]
enum Prism {
    MirrorLeftUp,
    MirrorLeftDown,
    SplitterHoriz,
    SplitterVert,
}

impl TileChar for Prism {
    fn to_char(&self) -> char {
        match *self {
            Self::MirrorLeftUp => '/',
            Self::MirrorLeftDown => '\\',
            Self::SplitterHoriz => '-',
            Self::SplitterVert => '|',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '/' => Some(Self::MirrorLeftUp),
            '\\' => Some(Self::MirrorLeftDown),
            '-' => Some(Self::SplitterHoriz),
            '|' => Some(Self::SplitterVert),
            _ => None,
        }
    }

    fn all_chars() -> Vec<char> {
        vec!['/', '\\', '-', '|']
    }
}

pub struct Contraption {
    tiles: TileMap<Prism>,
}

impl Contraption {
    fn energize(&self, start_p: Point2, start_dir: Cardinal) -> u64 {
        let range = self.tiles.get_range().unwrap();
        let mut beams: Vec<(Point2, Cardinal)> = vec![(start_p, start_dir)];
        let mut energized: HashSet<Point2> = HashSet::new();
        let mut already: HashSet<(Point2, Cardinal)> = HashSet::new();
        while let Some((p, dir)) = beams.pop() {
            if !already.insert((p, dir)) {
                continue;
            }
            if !range.contains(p) {
                continue;
            }
            energized.insert(p);

            match self.tiles.get(&p) {
                Some(Prism::MirrorLeftUp) => {
                    let next_dir = match dir {
                        Cardinal::North => Cardinal::East,
                        Cardinal::South => Cardinal::West,
                        Cardinal::East => Cardinal::North,
                        Cardinal::West => Cardinal::South,
                    };
                    let next_p = p.step(next_dir, 1);
                    beams.push((next_p, next_dir));
                }
                Some(Prism::MirrorLeftDown) => {
                    let next_dir = match dir {
                        Cardinal::North => Cardinal::West,
                        Cardinal::South => Cardinal::East,
                        Cardinal::East => Cardinal::South,
                        Cardinal::West => Cardinal::North,
                    };
                    let next_p = p.step(next_dir, 1);
                    beams.push((next_p, next_dir));
                }
                Some(Prism::SplitterHoriz) => match dir {
                    Cardinal::North | Cardinal::South => {
                        for next_dir in [Cardinal::East, Cardinal::West] {
                            beams.push((p.step(next_dir, 1), next_dir));
                        }
                    }
                    Cardinal::East | Cardinal::West => {
                        beams.push((p.step(dir, 1), dir));
                    }
                },
                Some(Prism::SplitterVert) => match dir {
                    Cardinal::East | Cardinal::West => {
                        for next_dir in [Cardinal::North, Cardinal::South] {
                            beams.push((p.step(next_dir, 1), next_dir));
                        }
                    }

                    Cardinal::North | Cardinal::South => {
                        beams.push((p.step(dir, 1), dir));
                    }
                },
                None => beams.push((p.step(dir, 1), dir)),
            }
        }
        energized.len() as u64
    }

    fn max_energize(&self) -> u64 {
        let range = self.tiles.get_range().unwrap();
        let top = (range.x.0..=range.x.1).map(|x| (Point2 { x, y: range.y.0 }, Cardinal::South));
        let bottom = (range.x.0..=range.x.1).map(|x| (Point2 { x, y: range.y.1 }, Cardinal::North));
        let left = (range.y.0..range.y.1).map(|y| (Point2 { x: range.x.0, y }, Cardinal::East));
        let right = (range.y.0..range.y.1).map(|y| (Point2 { x: range.x.1, y }, Cardinal::West));
        let all = top.chain(bottom).chain(left).chain(right);
        all.map(|(p, dir)| self.energize(p, dir)).max().unwrap()
    }
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let tiles = TileMap::from_string(value);
        Self { tiles }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Contraption {
    input.into()
}

#[aoc(day16, part1)]
pub fn part1(input: &Contraption) -> u64 {
    let value = input.energize(Point2::origin(), Cardinal::East);
    assert_eq!(value, 7979);
    value
}

#[aoc(day16, part2)]
pub fn part2(input: &Contraption) -> u64 {
    let value = input.max_energize();
    assert_eq!(value, 8437);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_energize() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.energize(Point2::origin(), Cardinal::East);
        assert_eq!(value, 46);
    }

    #[test]
    fn test_max_energize() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.max_energize();
        assert_eq!(value, 51);
    }
}
