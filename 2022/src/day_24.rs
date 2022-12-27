/*
    --- Day 24: Blizzard Basin ---
    With everything replanted for next year (and with elephants and monkeys to tend the grove), you and the Elves leave for the extraction point.

    Partway up the mountain that shields the grove is a flat, open area that serves as the extraction point. It's a bit of a climb, but nothing the expedition can't handle.

    At least, that would normally be true; now that the mountain is covered in snow, things have become more difficult than the Elves are used to.

    As the expedition reaches a valley that must be traversed to reach the extraction site, you find that strong, turbulent winds are pushing small blizzards of snow and sharp ice around the valley. It's a good thing everyone packed warm clothes! To make it across safely, you'll need to find a way to avoid them.

    Fortunately, it's easy to see all of this from the entrance to the valley, so you make a map of the valley and the blizzards (your puzzle input). For example:

    #.#####
    #.....#
    #>....#
    #.....#
    #...v.#
    #.....#
    #####.#
    The walls of the valley are drawn as #; everything else is ground. Clear ground - where there is currently no blizzard - is drawn as .. Otherwise, blizzards are drawn with an arrow indicating their direction of motion: up (^), down (v), left (<), or right (>).

    The above map includes two blizzards, one moving right (>) and one moving down (v). In one minute, each blizzard moves one position in the direction it is pointing:

    #.#####
    #.....#
    #.>...#
    #.....#
    #.....#
    #...v.#
    #####.#
    Due to conservation of blizzard energy, as a blizzard reaches the wall of the valley, a new blizzard forms on the opposite side of the valley moving in the same direction. After another minute, the bottom downward-moving blizzard has been replaced with a new downward-moving blizzard at the top of the valley instead:

    #.#####
    #...v.#
    #..>..#
    #.....#
    #.....#
    #.....#
    #####.#
    Because blizzards are made of tiny snowflakes, they pass right through each other. After another minute, both blizzards temporarily occupy the same position, marked 2:

    #.#####
    #.....#
    #...2.#
    #.....#
    #.....#
    #.....#
    #####.#
    After another minute, the situation resolves itself, giving each blizzard back its personal space:

    #.#####
    #.....#
    #....>#
    #...v.#
    #.....#
    #.....#
    #####.#
    Finally, after yet another minute, the rightward-facing blizzard on the right is replaced with a new one on the left facing the same direction:

    #.#####
    #.....#
    #>....#
    #.....#
    #...v.#
    #.....#
    #####.#
    This process repeats at least as long as you are observing it, but probably forever.

    Here is a more complex example:

    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#
    Your expedition begins in the only non-wall position in the top row and needs to reach the only non-wall position in the bottom row. On each minute, you can move up, down, left, or right, or you can wait in place. You and the blizzards act simultaneously, and you cannot share a position with a blizzard.

    In the above example, the fastest way to reach your goal requires 18 steps. Drawing the position of the expedition as E, one way to achieve this is:

    Initial state:
    #E######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#

    Minute 1, move down:
    #.######
    #E>3.<.#
    #<..<<.#
    #>2.22.#
    #>v..^<#
    ######.#

    Minute 2, move down:
    #.######
    #.2>2..#
    #E^22^<#
    #.>2.^>#
    #.>..<.#
    ######.#

    Minute 3, wait:
    #.######
    #<^<22.#
    #E2<.2.#
    #><2>..#
    #..><..#
    ######.#

    Minute 4, move up:
    #.######
    #E<..22#
    #<<.<..#
    #<2.>>.#
    #.^22^.#
    ######.#

    Minute 5, move right:
    #.######
    #2Ev.<>#
    #<.<..<#
    #.^>^22#
    #.2..2.#
    ######.#

    Minute 6, move right:
    #.######
    #>2E<.<#
    #.2v^2<#
    #>..>2>#
    #<....>#
    ######.#

    Minute 7, move down:
    #.######
    #.22^2.#
    #<vE<2.#
    #>>v<>.#
    #>....<#
    ######.#

    Minute 8, move left:
    #.######
    #.<>2^.#
    #.E<<.<#
    #.22..>#
    #.2v^2.#
    ######.#

    Minute 9, move up:
    #.######
    #<E2>>.#
    #.<<.<.#
    #>2>2^.#
    #.v><^.#
    ######.#

    Minute 10, move right:
    #.######
    #.2E.>2#
    #<2v2^.#
    #<>.>2.#
    #..<>..#
    ######.#

    Minute 11, wait:
    #.######
    #2^E^2>#
    #<v<.^<#
    #..2.>2#
    #.<..>.#
    ######.#

    Minute 12, move down:
    #.######
    #>>.<^<#
    #.<E.<<#
    #>v.><>#
    #<^v^^>#
    ######.#

    Minute 13, move down:
    #.######
    #.>3.<.#
    #<..<<.#
    #>2E22.#
    #>v..^<#
    ######.#

    Minute 14, move right:
    #.######
    #.2>2..#
    #.^22^<#
    #.>2E^>#
    #.>..<.#
    ######.#

    Minute 15, move right:
    #.######
    #<^<22.#
    #.2<.2.#
    #><2>E.#
    #..><..#
    ######.#

    Minute 16, move right:
    #.######
    #.<..22#
    #<<.<..#
    #<2.>>E#
    #.^22^.#
    ######.#

    Minute 17, move down:
    #.######
    #2.v.<>#
    #<.<..<#
    #.^>^22#
    #.2..2E#
    ######.#

    Minute 18, move down:
    #.######
    #>2.<.<#
    #.2v^2<#
    #>..>2>#
    #<....>#
    ######E#
    What is the fewest number of minutes required to avoid the blizzards and reach the goal?

    --- Part Two ---
    As the expedition reaches the far side of the valley, one of the Elves looks especially dismayed:

    He forgot his snacks at the entrance to the valley!

    Since you're so good at dodging blizzards, the Elves humbly request that you go back for his snacks. From the same initial conditions, how quickly can you make it from the start to the goal, then back to the start, then back to the goal?

    In the above example, the first trip to the goal takes 18 minutes, the trip back to the start takes 23 minutes, and the trip back to the goal again takes 13 minutes, for a total time of 54 minutes.

    What is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again?
*/

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    iter,
};

use common::{Cardinal, Mode, Point2, TileChar, TileMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Blizzard(u8), // Cardinal bitmask, 0 means the ground is clear
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Blizzard(x) => match x {
                1 => Cardinal::North.to_arrow(),
                2 => Cardinal::South.to_arrow(),
                4 => Cardinal::East.to_arrow(),
                8 => Cardinal::West.to_arrow(),
                _ => {
                    let ones = x.count_ones();
                    if ones == 0 {
                        '.'
                    } else {
                        (b'0' + x.count_ones() as u8) as char
                    }
                }
            },
        }
    }

    fn from_char(_c: char) -> Option<Self> {
        unimplemented!()
    }

    fn all_chars() -> Vec<char> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Valley {
    sim: TileMap<Tile>,
    time: usize,
    start: Point2,
    end: Point2,
    width: usize,
    height: usize,
}

impl Valley {
    fn from_str(s: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start = None;
        let mut end = None;
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            height = y;
            for (x, c) in line.chars().enumerate() {
                width = width.max(x);

                let pt = Point2 {
                    x: x as i32,
                    y: y as i32,
                };
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => {
                        if start.is_none() {
                            start = Some(pt);
                        }
                        end = Some(pt);
                        Tile::Blizzard(0)
                    }
                    _ => {
                        let cardinal = Cardinal::from_arrow(c);
                        Tile::Blizzard(1 << cardinal as u8)
                    }
                };
                tiles.insert(pt, tile);
            }
        }

        let sim = TileMap::new().with_tiles(tiles.into_iter());
        Self {
            sim,
            time: 0,
            start: start.unwrap(),
            end: end.unwrap(),
            width: width + 1,
            height: height + 1,
        }
    }

    fn advance_sim(&mut self) {
        // Create a fresh new state, keeping walls and walkable spaces
        let mut new_sim = self.sim.clone();
        for tile in new_sim.values_mut() {
            *tile = match tile {
                Tile::Wall => Tile::Wall,
                Tile::Blizzard(_) => Tile::Blizzard(0),
            }
        }

        for (pt, tile) in &*self.sim {
            if let Tile::Blizzard(blizz) = tile {
                for cardinal in Cardinal::all() {
                    let bit = 1 << cardinal as u8;
                    if *blizz & bit != 0 {
                        let adj_pt = pt.step(cardinal, 1);
                        if let Tile::Blizzard(adj_blizz) =
                            new_sim.entry(adj_pt).or_insert(Tile::Blizzard(0))
                        {
                            *adj_blizz |= bit;
                        } else {
                            // Assume no blizzards move / wrap into the start or end location (for the example and my input this isn't possible)
                            let adj_pt = match cardinal {
                                Cardinal::North => Point2 {
                                    x: pt.x,
                                    y: self.height as i32 - 2,
                                },
                                Cardinal::South => Point2 { x: pt.x, y: 1 },
                                Cardinal::East => Point2 { x: 1, y: pt.y },
                                Cardinal::West => Point2 {
                                    x: self.width as i32 - 2,
                                    y: pt.y,
                                },
                            };
                            let Tile::Blizzard(adj_blizz) = new_sim.entry(adj_pt).or_insert(Tile::Blizzard(0)) else { panic!("Should be blizzard") };
                            *adj_blizz |= bit;
                        }
                    }
                }
            }
        }
        self.sim = new_sim;
        self.time += 1;
    }

    fn find_end(&mut self, mode: Mode) -> usize {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum Goal {
            End,
            Snacks,
            EndAgain,
        }

        let mut frontier: Vec<(usize, Point2, Goal)> = vec![(0, self.start, Goal::End)]; // Time, location, goal
        let mut visited: HashSet<(usize, Point2, Goal)> = HashSet::new();
        while let Some((time, curr_pt, state)) = frontier.pop() {
            while time >= self.time {
                self.advance_sim();
            }

            for next_pt in curr_pt.orthogonals().chain(iter::once(curr_pt)) {
                if Some(&Tile::Blizzard(0)) == self.sim.get(&next_pt) {
                    // It's OK to break early when reaching the goal for each stage, because it'll never be shorter
                    // to take longer to get to the goal (we could always get there as fast as possible and then sit there doing nothing)
                    match state {
                        Goal::End if next_pt == self.end => {
                            if mode == Mode::M1 {
                                return time + 1;
                            }

                            let next = (time + 1, next_pt, Goal::Snacks);
                            visited.clear();
                            frontier.clear();
                            visited.insert(next);
                            frontier.push(next);
                            break;
                        }
                        Goal::Snacks if next_pt == self.start => {
                            let next = (time + 1, next_pt, Goal::EndAgain);
                            visited.clear();
                            frontier.clear();
                            visited.insert(next);
                            frontier.push(next);
                            break;
                        }
                        Goal::EndAgain if next_pt == self.end => {
                            return time + 1;
                        }
                        _ => {
                            let next = (time + 1, next_pt, state);
                            if !visited.contains(&next) {
                                visited.insert(next);
                                frontier.push(next);
                            }
                        }
                    }
                }
            }

            frontier.sort_unstable_by_key(|(time, _, _)| Reverse(*time));
        }
        panic!("Couldn't find the goal");
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Valley {
    Valley::from_str(input)
}

#[aoc(day24, part1)]
pub fn part1(input: &Valley) -> usize {
    let mut valley = input.clone();
    let end = valley.find_end(Mode::M1);
    assert_eq!(end, 232);
    end
}

#[aoc(day24, part2)]
pub fn part2(input: &Valley) -> usize {
    let mut valley = input.clone();
    let end = valley.find_end(Mode::M2);
    assert_eq!(end, 715);
    end
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_find_goal() {
        let input = input_generator(EXAMPLE_INPUT);

        let mut valley = input.clone();
        let end = valley.find_end(Mode::M1);
        assert_eq!(end, 18);

        let mut valley = input.clone();
        let end = valley.find_end(Mode::M2);
        assert_eq!(end, 54);
    }
}
