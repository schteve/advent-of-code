/*

*/

use std::collections::HashSet;

use common::{Cardinal, Point2, TileChar, TileMap};

#[derive(Clone, Copy, Default)]
pub struct Pipe {
    bits: u8, // NSEW, written in binary as 0bWESN
}

impl Pipe {
    fn from_dirs<I>(dirs: I) -> Self
    where
        I: IntoIterator<Item = Cardinal>,
    {
        let mut bits = 0;
        for dir in dirs.into_iter() {
            let bit = match dir {
                Cardinal::North => 0,
                Cardinal::South => 1,
                Cardinal::East => 2,
                Cardinal::West => 3,
            };
            bits |= 1 << bit;
        }
        Self { bits }
    }

    fn is_connection(&self, dir: Cardinal) -> bool {
        let bit = match dir {
            Cardinal::North => 0,
            Cardinal::South => 1,
            Cardinal::East => 2,
            Cardinal::West => 3,
        };
        (self.bits & (1 << bit)) != 0
    }

    fn connections(&self) -> impl Iterator<Item = Cardinal> + '_ {
        Cardinal::all().filter(|dir| self.is_connection(*dir))
    }
}

impl TileChar for Pipe {
    fn to_char(&self) -> char {
        match self.bits {
            0b0000 => '.',
            0b0011 => '|',
            0b1100 => '-',
            0b0101 => 'L',
            0b1001 => 'J',
            0b1010 => '7',
            0b0110 => 'F',
            0b1111 => 'S',
            _ => panic!("Unknown bits: {:#04b}", self.bits),
        }
    }

    fn from_char(c: char) -> Option<Self> {
        let bits = match c {
            '.' => 0b0000,
            '|' => 0b0011,
            '-' => 0b1100,
            'L' => 0b0101,
            'J' => 0b1001,
            '7' => 0b1010,
            'F' => 0b0110,
            'S' => 0b1111,
            _ => return None,
        };

        Some(Self { bits })
    }

    fn all_chars() -> Vec<char> {
        unimplemented!()
    }
}

pub struct Map {
    map: TileMap<Pipe>,
    start: Point2,
}

impl Map {
    fn follow_the_white_rabbit(&self) -> HashSet<Point2> {
        let mut traveled = HashSet::from([self.start]);
        let start_paths = self
            .map
            .get(&self.start)
            .unwrap()
            .connections()
            .map(|dir| self.start.step(dir, 1));
        for path in start_paths {
            let mut curr = path;
            while !traveled.contains(&curr) {
                traveled.insert(curr);
                if let Some(next) = self.next_point(curr, &traveled) {
                    curr = next;
                } else {
                    break;
                }
            }
        }
        traveled
    }

    fn next_point(&self, curr: Point2, traveled: &HashSet<Point2>) -> Option<Point2> {
        let mut paths = self
            .map
            .get(&curr)
            .unwrap()
            .connections()
            .filter_map(|dir| {
                let adj = curr.step(dir, 1);
                if traveled.contains(&adj) {
                    None
                } else {
                    Some(adj)
                }
            });
        paths.next()
    }

    fn enclosed(&self) -> usize {
        let mut count = 0;

        // We can know if we're inside a polygon by scanning each row and
        // counting edge transitions. Here, an edge is only transited once both
        // transverse directions are seen. Otherwise, we could be skimming the
        // edge but not entering.
        let path = self.follow_the_white_rabbit();
        let range = Point2::get_range(path.iter()).unwrap();
        for y in range.y.0..=range.y.1 {
            let mut inside = false;
            let mut north_bit = false;
            let mut south_bit = false;
            for x in range.x.0..=range.x.1 {
                let p = Point2 { x, y };
                if path.contains(&p) {
                    let pipe = self.map.get(&p).unwrap();
                    north_bit ^= pipe.is_connection(Cardinal::North);
                    south_bit ^= pipe.is_connection(Cardinal::South);
                    if north_bit && south_bit {
                        inside = !inside;
                        north_bit = false;
                        south_bit = false;
                    }
                    //print!("{}", pipe.to_char());
                } else if inside {
                    count += 1;
                    //print!("\x1b[32mI\x1b[m");
                } else {
                    //print!("\x1b[31mO\x1b[m");
                }
            }
            //println!("");
        }

        count
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut map: TileMap<Pipe, '.'> = TileMap::from_string(value);
        let start = map
            .iter()
            .find_map(|(point, pipe)| {
                if pipe.to_char() == 'S' {
                    Some(*point)
                } else {
                    None
                }
            })
            .unwrap();

        // The start pipe is really just a mirror of what's around it...
        // figure that out and replace it in the map.
        let start_dirs = Cardinal::all().filter(|dir| {
            let adj = start.step(*dir, 1);
            if let Some(adj_pipe) = map.get(&adj) {
                adj_pipe.is_connection(dir.opposite())
            } else {
                false
            }
        });
        let start_pipe = Pipe::from_dirs(start_dirs);
        map.insert(start, start_pipe);

        Self { map, start }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    input.into()
}

#[aoc(day10, part1)]
pub fn part1(input: &Map) -> usize {
    let value = input.follow_the_white_rabbit().len() / 2;
    assert_eq!(value, 6831);
    value
}

#[aoc(day10, part2)]
pub fn part2(input: &Map) -> usize {
    let value = input.enclosed();
    assert_eq!(value, 305);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    static EXAMPLE_INPUT_2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    static EXAMPLE_INPUT_3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    static EXAMPLE_INPUT_4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    static EXAMPLE_INPUT_5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_follow_the_white_rabbit() {
        let expected = [(EXAMPLE_INPUT_1, 4), (EXAMPLE_INPUT_2, 8)];
        for (input, output) in expected {
            let input = input_generator(input);
            let value = input.follow_the_white_rabbit().len() / 2;
            assert_eq!(value, output);
        }
    }

    #[test]
    fn test_enclosed() {
        let expected = [
            (EXAMPLE_INPUT_1, 1),
            (EXAMPLE_INPUT_2, 1),
            (EXAMPLE_INPUT_3, 4),
            (EXAMPLE_INPUT_4, 8),
            (EXAMPLE_INPUT_5, 10),
        ];
        for (input, output) in expected {
            let input = input_generator(input);
            let value = input.enclosed();
            assert_eq!(value, output);
        }
    }
}
