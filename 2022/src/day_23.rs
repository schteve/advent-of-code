/*
    --- Day 23: Unstable Diffusion ---
    You enter a large crater of gray dirt where the grove is supposed to be. All around you, plants you imagine were expected to be full of fruit are instead withered and broken. A large group of Elves has formed in the middle of the grove.

    "...but this volcano has been dormant for months. Without ash, the fruit can't grow!"

    You look up to see a massive, snow-capped mountain towering above you.

    "It's not like there are other active volcanoes here; we've looked everywhere."

    "But our scanners show active magma flows; clearly it's going somewhere."

    They finally notice you at the edge of the grove, your pack almost overflowing from the random star fruit you've been collecting. Behind you, elephants and monkeys explore the grove, looking concerned. Then, the Elves recognize the ash cloud slowly spreading above your recent detour.

    "Why do you--" "How is--" "Did you just--"

    Before any of them can form a complete question, another Elf speaks up: "Okay, new plan. We have almost enough fruit already, and ash from the plume should spread here eventually. If we quickly plant new seedlings now, we can still make it to the extraction point. Spread out!"

    The Elves each reach into their pack and pull out a tiny plant. The plants rely on important nutrients from the ash, so they can't be planted too close together.

    There isn't enough time to let the Elves figure out where to plant the seedlings themselves; you quickly scan the grove (your puzzle input) and note their positions.

    For example:

    ....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..
    The scan shows Elves # and empty ground .; outside your scan, more empty ground extends a long way in every direction. The scan is oriented so that north is up; orthogonal directions are written N (north), S (south), W (west), and E (east), while diagonal directions are written NE, NW, SE, SW.

    The Elves follow a time-consuming process to figure out where they should each go; you can speed up this process considerably. The process consists of some number of rounds during which Elves alternate between considering where to move and actually moving.

    During the first half of each round, each Elf considers the eight positions adjacent to themself. If no other Elves are in one of those eight positions, the Elf does not do anything during this round. Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:

    If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
    After each Elf has had a chance to propose a move, the second half of the round can begin. Simultaneously, each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position. If two or more Elves propose moving to the same position, none of those Elves move.

    Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions. For example, during the second round, the Elves would try proposing a move to the south first, then west, then east, then north. On the third round, the Elves would first consider west, then east, then north, then south.

    As a smaller example, consider just these five Elves:

    .....
    ..##.
    ..#..
    .....
    ..##.
    .....
    The northernmost two Elves and southernmost two Elves all propose moving north, while the middle Elf cannot move north and proposes moving south. The middle Elf proposes the same destination as the southwest Elf, so neither of them move, but the other three do:

    ..##.
    .....
    ..#..
    ...#.
    ..#..
    .....
    Next, the northernmost two Elves and the southernmost Elf all propose moving south. Of the remaining middle two Elves, the west one cannot move south and proposes moving west, while the east one cannot move south or west and proposes moving east. All five Elves succeed in moving to their proposed positions:

    .....
    ..##.
    .#...
    ....#
    .....
    ..#..
    Finally, the southernmost two Elves choose not to move at all. Of the remaining three Elves, the west one proposes moving west, the east one proposes moving east, and the middle one proposes moving north; all three succeed in moving:

    ..#..
    ....#
    #....
    ....#
    .....
    ..#..
    At this point, no Elves need to move, and so the process ends.

    The larger example above proceeds as follows:

    == Initial State ==
    ..............
    ..............
    .......#......
    .....###.#....
    ...#...#.#....
    ....#...##....
    ...#.###......
    ...##.#.##....
    ....#..#......
    ..............
    ..............
    ..............

    == End of Round 1 ==
    ..............
    .......#......
    .....#...#....
    ...#..#.#.....
    .......#..#...
    ....#.#.##....
    ..#..#.#......
    ..#.#.#.##....
    ..............
    ....#..#......
    ..............
    ..............

    == End of Round 2 ==
    ..............
    .......#......
    ....#.....#...
    ...#..#.#.....
    .......#...#..
    ...#..#.#.....
    .#...#.#.#....
    ..............
    ..#.#.#.##....
    ....#..#......
    ..............
    ..............

    == End of Round 3 ==
    ..............
    .......#......
    .....#....#...
    ..#..#...#....
    .......#...#..
    ...#..#.#.....
    .#..#.....#...
    .......##.....
    ..##.#....#...
    ...#..........
    .......#......
    ..............

    == End of Round 4 ==
    ..............
    .......#......
    ......#....#..
    ..#...##......
    ...#.....#.#..
    .........#....
    .#...###..#...
    ..#......#....
    ....##....#...
    ....#.........
    .......#......
    ..............

    == End of Round 5 ==
    .......#......
    ..............
    ..#..#.....#..
    .........#....
    ......##...#..
    .#.#.####.....
    ...........#..
    ....##..#.....
    ..#...........
    ..........#...
    ....#..#......
    ..............
    After a few more rounds...

    == End of Round 10 ==
    .......#......
    ...........#..
    ..#.#..#......
    ......#.......
    ...#.....#..#.
    .#......##....
    .....##.......
    ..#........#..
    ....#.#..#....
    ..............
    ....#..#..#...
    ..............
    To make sure they're on the right track, the Elves like to check after round 10 that they're making good progress toward covering enough ground. To do this, count the number of empty ground tiles contained by the smallest rectangle that contains every Elf. (The edges of the rectangle should be aligned to the N/S/E/W directions; the Elves do not have the patience to calculate arbitrary rectangles.) In the above example, that rectangle is:

    ......#.....
    ..........#.
    .#.#..#.....
    .....#......
    ..#.....#..#
    #......##...
    ....##......
    .#........#.
    ...#.#..#...
    ............
    ...#..#..#..
    In this region, the number of empty ground tiles is 110.

    Simulate the Elves' process and find the smallest rectangle that contains the Elves after 10 rounds. How many empty ground tiles does that rectangle contain?

    --- Part Two ---
    It seems you're on the right track. Finish simulating the process and figure out where the Elves need to go. How many rounds did you save them?

    In the example above, the first round where no Elf moved was round 20:

    .......#......
    ....#......#..
    ..#.....#.....
    ......#.......
    ...#....#.#..#
    #.............
    ....#.....#...
    ..#.....#.....
    ....#.#....#..
    .........#....
    ....#......#..
    .......#......
    Figure out where the Elves need to go. What is the number of the first round where no Elf moves?
*/

use std::collections::HashMap;

use common::{Cardinal, Point2, TileSet};

const NW: Point2 = Point2 { x: -1, y: -1 };
const N: Point2 = Point2 { x: 0, y: -1 };
const NE: Point2 = Point2 { x: 1, y: -1 };
const E: Point2 = Point2 { x: 1, y: 0 };
const SE: Point2 = Point2 { x: 1, y: 1 };
const S: Point2 = Point2 { x: 0, y: 1 };
const SW: Point2 = Point2 { x: -1, y: 1 };
const W: Point2 = Point2 { x: -1, y: 0 };

const ADJACENT_SIDES: [(Cardinal, [Point2; 3]); 4] = [
    (Cardinal::North, [NW, N, NE]),
    (Cardinal::South, [SW, S, SE]),
    (Cardinal::West, [NW, W, SW]),
    (Cardinal::East, [NE, E, SE]),
];

#[derive(Clone)]
pub struct Grove {
    map: TileSet,
}

impl Grove {
    fn step(&mut self, proposed: &mut HashMap<Point2, Option<Point2>>, round: usize) -> usize {
        // Proposed point -> proposed by who (None = more than one elf, Some = one elf)
        proposed.clear();
        for elf in self.map.iter() {
            if !elf.adjacents().any(|adj| self.map.contains(&adj)) {
                // No adjacent elves, don't do anything
                continue;
            }

            let first_adj = ADJACENT_SIDES
                .into_iter()
                .cycle()
                .skip(round % 4)
                .take(4)
                .map(|(dir, side)| (dir, side.map(|s| s + elf)))
                .find(|(_dir, [a, b, c])| {
                    !(self.map.contains(a) || self.map.contains(b) || self.map.contains(c))
                });
            if let Some((dir, _)) = first_adj {
                let proposed_pt = elf.step(dir, 1);
                let entry = proposed.entry(proposed_pt).or_insert(Some(*elf));
                if *entry != Some(*elf) {
                    // Entry already existed, now more than one elf proposed this point so wipe it to None
                    *entry = None;
                }
            }
        }

        for (pt, who) in proposed.iter() {
            if let Some(elf) = who {
                // This move was proposed by one elf - move them
                self.map.remove(elf);
                self.map.insert(*pt);
            }
        }

        proposed.len()
    }

    fn diffuse(&mut self, n: usize) -> u64 {
        let mut proposed = HashMap::new();
        for round in 0..n {
            self.step(&mut proposed, round);
        }

        let range = self.map.get_range().unwrap();
        let area = (range.x.1 - range.x.0 + 1) * (range.y.1 - range.y.0 + 1);
        area as u64 - self.map.len() as u64
    }

    fn diffuse_until_stable(&mut self) -> usize {
        let mut proposed = HashMap::new();

        for round in 0.. {
            let moves = self.step(&mut proposed, round);
            if moves == 0 {
                return round + 1; // Need 1-based
            }
        }

        unreachable!()
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Grove {
    Grove {
        map: TileSet::from_string::<'#'>(input),
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Grove) -> u64 {
    let mut grove = input.clone();
    let empty = grove.diffuse(10);
    assert_eq!(empty, 4070);
    empty
}

#[aoc(day23, part2)]
pub fn part2(input: &Grove) -> usize {
    let mut grove = input.clone();
    let rounds = grove.diffuse_until_stable();
    assert_eq!(rounds, 881);
    rounds
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_diffuse() {
        let mut input = input_generator(EXAMPLE_INPUT);
        let empty = input.diffuse(10);
        assert_eq!(empty, 110);

        let expected = "\
......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..";
        assert_eq!(input.map.to_string().trim(), expected);
    }

    #[test]
    fn test_diffuse_until_stable() {
        let mut input = input_generator(EXAMPLE_INPUT);
        let rounds = input.diffuse_until_stable();
        assert_eq!(rounds, 20);
    }
}
