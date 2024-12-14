/*
    --- Day 10: Hoof It ---
    You all arrive at a Lava Production Facility on a floating island in the sky. As the others begin to search the massive industrial complex, you feel a small nose boop your leg and look down to discover a reindeer wearing a hard hat.

    The reindeer is holding a book titled "Lava Island Hiking Guide". However, when you open the book, you discover that most of it seems to have been scorched by lava! As you're about to ask how you can help, the reindeer brings you a blank topographic map of the surrounding area (your puzzle input) and looks up at you excitedly.

    Perhaps you can help fill in the missing hiking trails?

    The topographic map indicates the height at each position using a scale from 0 (lowest) to 9 (highest). For example:

    0123
    1234
    8765
    9876
    Based on un-scorched scraps of the book, you determine that a good hiking trail is as long as possible and has an even, gradual, uphill slope. For all practical purposes, this means that a hiking trail is any path that starts at height 0, ends at height 9, and always increases by a height of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left, or right (from the perspective of the map).

    You look up from the map and notice that the reindeer has helpfully begun to construct a small pile of pencils, markers, rulers, compasses, stickers, and other equipment you might need to update the map with hiking trails.

    A trailhead is any position that starts one or more hiking trails - here, these positions will always have height 0. Assembling more fragments of pages, you establish that a trailhead's score is the number of 9-height positions reachable from that trailhead via a hiking trail. In the above example, the single trailhead in the top left corner has a score of 1 because it can reach a single 9 (the one in the bottom left).

    This trailhead has a score of 2:

    ...0...
    ...1...
    ...2...
    6543456
    7.....7
    8.....8
    9.....9
    (The positions marked . are impassable tiles to simplify these examples; they do not appear on your actual topographic map.)

    This trailhead has a score of 4 because every 9 is reachable via a hiking trail except the one immediately to the left of the trailhead:

    ..90..9
    ...1.98
    ...2..7
    6543456
    765.987
    876....
    987....
    This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2:

    10..9..
    2...8..
    3...7..
    4567654
    ...8..3
    ...9..2
    .....01
    Here's a larger example:

    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    This larger example has 9 trailheads. Considering the trailheads in reading order, they have scores of 5, 6, 5, 3, 1, 3, 5, 3, and 5. Adding these scores together, the sum of the scores of all trailheads is 36.

    The reindeer gleefully carries over a protractor and adds it to the pile. What is the sum of the scores of all trailheads on your topographic map?

    --- Part Two ---
    The reindeer spends a few minutes reviewing your hiking trail map before realizing something, disappearing for a few minutes, and finally returning with yet another slightly-charred piece of paper.

    The paper describes a second way to measure a trailhead called its rating. A trailhead's rating is the number of distinct hiking trails which begin at that trailhead. For example:

    .....0.
    ..4321.
    ..5..2.
    ..6543.
    ..7..4.
    ..8765.
    ..9....
    The above map has a single trailhead; its rating is 3 because there are exactly three distinct hiking trails which begin at that position:

    .....0.   .....0.   .....0.
    ..4321.   .....1.   .....1.
    ..5....   .....2.   .....2.
    ..6....   ..6543.   .....3.
    ..7....   ..7....   .....4.
    ..8....   ..8....   ..8765.
    ..9....   ..9....   ..9....
    Here is a map containing a single trailhead with rating 13:

    ..90..9
    ...1.98
    ...2..7
    6543456
    765.987
    876....
    987....
    This map contains a single trailhead with rating 227 (because there are 121 distinct hiking trails that lead to the 9 on the right edge and 106 that lead to the 9 on the bottom edge):

    012345
    123456
    234567
    345678
    4.6789
    56789.
    Here's the larger example from before:

    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    Considering its trailheads in reading order, they have ratings of 20, 24, 10, 4, 1, 4, 5, 8, and 5. The sum of all trailhead ratings in this larger example topographic map is 81.

    You're not sure how, but the reindeer seems to have crafted some tiny flags out of toothpicks and bits of paper and is using them to mark trailheads on your topographic map. What is the sum of the ratings of all trailheads?
*/

use std::collections::{HashMap, HashSet};

use common::Point2;

pub struct Topo {
    tiles: HashMap<Point2, u32>,
}

impl Topo {
    fn parse(input: &str) -> Self {
        let mut tiles = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point2 {
                    x: x as i32,
                    y: y as i32,
                };
                if let Some(digit) = c.to_digit(10) {
                    tiles.insert(p, digit);
                }
            }
        }

        Self { tiles }
    }

    fn trailhead_score(&self) -> u64 {
        self.tiles
            .iter()
            .filter_map(|(p, height)| {
                if *height == 0 {
                    let mut peak_set = HashSet::new();
                    self.peaks_from(*p, &mut peak_set);
                    Some(peak_set.len() as u64)
                } else {
                    None
                }
            })
            .sum()
    }

    fn trailhead_rating(&self) -> u64 {
        self.tiles
            .iter()
            .filter_map(|(p, height)| {
                if *height == 0 {
                    let mut peak_set = HashSet::new();
                    Some(self.peaks_from(*p, &mut peak_set))
                } else {
                    None
                }
            })
            .sum()
    }

    fn peaks_from(&self, p: Point2, peaks: &mut HashSet<Point2>) -> u64 {
        let height = self.tiles.get(&p).unwrap();
        if *height == 9 {
            peaks.insert(p);
            1
        } else {
            p.orthogonals()
                .map(|adj| {
                    if self.tiles.get(&adj) == Some(&(height + 1)) {
                        self.peaks_from(adj, peaks)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Topo {
    Topo::parse(input)
}

#[aoc(day10, part1)]
pub fn part1(input: &Topo) -> u64 {
    let value = input.trailhead_score();
    assert_eq!(value, 789);
    value
}

#[aoc(day10, part2)]
pub fn part2(input: &Topo) -> u64 {
    let value = input.trailhead_rating();
    assert_eq!(value, 1735);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str = "\
0123
1234
8765
9876";

    static EXAMPLE_INPUT_2: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    static EXAMPLE_INPUT_3: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    static EXAMPLE_INPUT_4: &str = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

    static EXAMPLE_INPUT_5: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    static EXAMPLE_INPUT_6: &str = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

    static EXAMPLE_INPUT_7: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    static EXAMPLE_INPUT_8: &str = "\
012345
123456
234567
345678
4.6789
56789.";

    #[test]
    fn test_trailhead_score() {
        let test_data = [
            (EXAMPLE_INPUT_1, 1),
            (EXAMPLE_INPUT_2, 2),
            (EXAMPLE_INPUT_3, 4),
            (EXAMPLE_INPUT_4, 3),
            (EXAMPLE_INPUT_5, 36),
        ];
        for (input, expected) in test_data {
            let input = input_generator(input);
            let value = input.trailhead_score();
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn test_trailhead_rating() {
        let test_data = [
            (EXAMPLE_INPUT_5, 81),
            (EXAMPLE_INPUT_6, 3),
            (EXAMPLE_INPUT_7, 13),
            (EXAMPLE_INPUT_8, 227),
        ];
        for (input, expected) in test_data {
            let input = input_generator(input);
            let value = input.trailhead_rating();
            assert_eq!(value, expected);
        }
    }
}
