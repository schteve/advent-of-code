/*
    --- Day 24: Lobby Layout ---
    Your raft makes it to the tropical island; it turns out that the small crab was an excellent navigator. You make your way to the resort.

    As you enter the lobby, you discover a small problem: the floor is being renovated. You can't even reach the check-in desk until they've finished installing the new tile floor.

    The tiles are all hexagonal; they need to be arranged in a hex grid with a very specific color pattern. Not in the mood to wait, you offer to help figure out the pattern.

    The tiles are all white on one side and black on the other. They start with the white side facing up. The lobby is large enough to fit whatever pattern might need to appear there.

    A member of the renovation crew gives you a list of the tiles that need to be flipped over (your puzzle input). Each line in the list identifies a single tile that needs to be flipped by giving a series of steps starting from a reference tile in the very center of the room. (Every line starts from the same reference tile.)

    Because the tiles are hexagonal, every tile has six neighbors: east, southeast, southwest, west, northwest, and northeast. These directions are given in your list, respectively, as e, se, sw, w, nw, and ne. A tile is identified by a series of these directions with no delimiters; for example, esenee identifies the tile you land on if you start at the reference tile and then move one tile east, one tile southeast, one tile northeast, and one tile east.

    Each time a tile is identified, it flips from white to black or from black to white. Tiles might be flipped more than once. For example, a line like esew flips a tile immediately adjacent to the reference tile, and a line like nwwswee flips the reference tile itself.

    Here is a larger example:

    sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew
    In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to black, then back to white). After all of these instructions have been followed, a total of 10 tiles are black.

    Go through the renovation crew's list and determine which tiles they need to flip. After all of the instructions have been followed, how many tiles are left with the black side up?

    --- Part Two ---
    The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all flipped according to the following rules:

    Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    Here, tiles immediately adjacent means the six tiles directly touching the tile in question.

    The rules are applied simultaneously to every tile; put another way, it is first determined which tiles need to be flipped, then they are all flipped at the same time.

    In the above example, the number of black tiles that are facing up after the given number of days has passed is as follows:

    Day 1: 15
    Day 2: 12
    Day 3: 25
    Day 4: 14
    Day 5: 23
    Day 6: 28
    Day 7: 41
    Day 8: 37
    Day 9: 49
    Day 10: 37

    Day 20: 132
    Day 30: 259
    Day 40: 406
    Day 50: 566
    Day 60: 788
    Day 70: 1106
    Day 80: 1373
    Day 90: 1844
    Day 100: 2208
    After executing this process a total of 100 times, there would be 2208 black tiles facing up.

    How many tiles will be black after 100 days?
*/

use crate::common::trim_start;
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum HexDir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl HexDir {
    fn parser(input: &str) -> IResult<&str, Self> {
        trim_start(alt((
            value(Self::East, tag("e")),
            value(Self::SouthEast, tag("se")),
            value(Self::SouthWest, tag("sw")),
            value(Self::West, tag("w")),
            value(Self::NorthWest, tag("nw")),
            value(Self::NorthEast, tag("ne")),
        )))(input)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct HexPoint {
    x: i8,
    y: i8,
    z: i8,
}

impl HexPoint {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn step(&self, dir: &HexDir) -> Self {
        let (dx, dy, dz) = match dir {
            HexDir::East => (1, -1, 0),
            HexDir::SouthEast => (0, -1, 1),
            HexDir::SouthWest => (-1, 0, 1),
            HexDir::West => (-1, 1, 0),
            HexDir::NorthWest => (0, 1, -1),
            HexDir::NorthEast => (1, 0, -1),
        };

        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    fn adjacents(&self) -> [Self; 6] {
        [
            self.step(&HexDir::East),
            self.step(&HexDir::SouthEast),
            self.step(&HexDir::SouthWest),
            self.step(&HexDir::West),
            self.step(&HexDir::NorthWest),
            self.step(&HexDir::NorthEast),
        ]
    }
}

struct Floor {
    tiles: HashSet<HexPoint>,
}

impl Floor {
    fn from_rules(rules: &[Vec<HexDir>]) -> Self {
        let mut tiles = HashSet::new();
        for rule in rules {
            // Walk through the directions
            let mut point = HexPoint::new();
            for dir in rule {
                point = point.step(dir);
            }

            // Toggle the final tile
            if tiles.contains(&point) == true {
                tiles.remove(&point);
            } else {
                tiles.insert(point);
            }
        }

        Self { tiles }
    }

    fn count_black(&self) -> usize {
        self.tiles.len()
    }

    fn days_passed(&mut self, num: usize) {
        let mut counts: HashMap<HexPoint, u8> = HashMap::new();
        for _ in 0..num {
            counts.clear();
            for t in &self.tiles {
                for adj in &t.adjacents() {
                    let e = counts.entry(*adj).or_insert(0);
                    *e += 1;
                }
            }

            self.tiles = counts
                .iter()
                .filter_map(|(&tile, &count)| {
                    if self.tiles.contains(&tile) {
                        // Black tile rules
                        if count == 0 || count > 2 {
                            None // Flip to white (don't do anything)
                        } else {
                            Some(tile) // Keep as black
                        }
                    } else {
                        // White tile rules
                        if count == 2 {
                            Some(tile) // Flip to black
                        } else {
                            None // Keep as white (don't do anything)
                        }
                    }
                })
                .collect();
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Vec<HexDir>> {
    input
        .lines()
        .map(|line| many1(HexDir::parser)(line).unwrap().1)
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Vec<HexDir>]) -> usize {
    let floor = Floor::from_rules(input);
    let black = floor.count_black();
    assert_eq!(black, 469);
    black
}

#[aoc(day24, part2)]
pub fn part2(input: &[Vec<HexDir>]) -> usize {
    let mut floor = Floor::from_rules(input);
    floor.days_passed(100);
    let black = floor.count_black();
    assert_eq!(black, 4353);
    black
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_setup() {
        let input = input_generator(EXAMPLE_INPUT);
        let floor = Floor::from_rules(&input);
        assert_eq!(floor.count_black(), 10);
    }

    #[test]
    #[ignore]
    fn test_days_passed() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut floor = Floor::from_rules(&input);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 15);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 12);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 25);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 14);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 23);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 28);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 41);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 37);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 49);

        floor.days_passed(1);
        assert_eq!(floor.count_black(), 37);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 132);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 259);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 406);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 566);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 788);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 1106);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 1373);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 1844);

        floor.days_passed(10);
        assert_eq!(floor.count_black(), 2208);

        let mut floor = Floor::from_rules(&input);
        floor.days_passed(100);
        assert_eq!(floor.count_black(), 2208);
    }
}
