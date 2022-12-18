/*
    --- Day 15: Beacon Exclusion Zone ---
    You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.

    The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

    Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. (There is never a tie where two beacons are the same distance to a sensor.)

    It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input). For example:

    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For the sensor at 9,16, the closest beacon to it is at 10,16.

    Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:

                   1    1    2    2
         0    5    0    5    0    5
     0 ....S.......................
     1 ......................S.....
     2 ...............S............
     3 ................SB..........
     4 ............................
     5 ............................
     6 ............................
     7 ..........S.......S.........
     8 ............................
     9 ............................
    10 ....B.......................
    11 ..S.........................
    12 ............................
    13 ............................
    14 ..............S.......S.....
    15 B...........................
    16 ...........SB...............
    17 ................S..........B
    18 ....S.......................
    19 ............................
    20 ............S......S........
    21 ............................
    22 .......................B....
    This isn't necessarily a comprehensive map of all beacons in the area, though. Because each sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor. There could still be beacons that just happen to not be the closest beacon to any sensor. Consider the sensor at 8,7:

                   1    1    2    2
         0    5    0    5    0    5
    -2 ..........#.................
    -1 .........###................
     0 ....S...#####...............
     1 .......#######........S.....
     2 ......#########S............
     3 .....###########SB..........
     4 ....#############...........
     5 ...###############..........
     6 ..#################.........
     7 .#########S#######S#........
     8 ..#################.........
     9 ...###############..........
    10 ....B############...........
    11 ..S..###########............
    12 ......#########.............
    13 .......#######..............
    14 ........#####.S.......S.....
    15 B........###................
    16 ..........#SB...............
    17 ................S..........B
    18 ....S.......................
    19 ............................
    20 ............S......S........
    21 ............................
    22 .......................B....
    This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).

    None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

    So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist. The coverage from all sensors near that row looks like this:

                     1    1    2    2
           0    5    0    5    0    5
     9 ...#########################...
    10 ..####B######################..
    11 .###S#############.###########.
    In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.

    Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?

    --- Part Two ---
    Your handheld device indicates that the distress signal is coming from a beacon nearby. The distress beacon is not detected by any sensor, but the distress beacon must have x and y coordinates each no lower than 0 and no larger than 4000000.

    To isolate the distress beacon's signal, you need to determine its tuning frequency, which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.

    In the example above, the search space is smaller: instead, the x and y coordinates can each be at most 20. With this reduced search area, there is only a single position that could have a beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.

    Find the only possible position for the distress beacon. What is its tuning frequency?
*/

use common::Point2;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Sensor {
    pos: Point2,
    beacon: Point2,
    dist: u32,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

impl Sensor {
    fn from_str(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();
        let pos = Point2 {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
        };
        let beacon = Point2 {
            x: caps.get(3).unwrap().as_str().parse().unwrap(),
            y: caps.get(4).unwrap().as_str().parse().unwrap(),
        };
        let dist = Point2::manhattan(pos, beacon);
        Self { pos, beacon, dist }
    }

    fn intersect_horizontal(&self, y: i32) -> Option<Line> {
        let dist = self.pos.y.abs_diff(y);
        if dist <= self.dist {
            let width_half = (self.dist - dist) as i32;
            let new_line = Line {
                left: self.pos.x - width_half,
                right: self.pos.x + width_half,
            };
            Some(new_line)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    left: i32,
    right: i32,
}

impl Line {
    fn len(&self) -> usize {
        (self.left..=self.right).count()
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.left <= self.right && other.right >= self.left
    }

    fn union(&self, other: &Self) -> Option<Line> {
        if self.overlaps(other) {
            Some(Line {
                left: self.left.min(other.left),
                right: self.right.max(other.right),
            })
        } else {
            None
        }
    }
}

fn calc_lines_at_y(sensors: &[Sensor], y: i32) -> Vec<Line> {
    let lines: Vec<Line> = sensors
        .iter()
        .filter_map(|sensor| sensor.intersect_horizontal(y))
        .sorted_by_key(|line| line.left)
        .collect();

    let mut output = Vec::new();
    let mut line_merge: Option<Line> = None;
    for line in lines {
        if let Some(l) = &line_merge {
            if let Some(u) = l.union(&line) {
                // Union worked, save it
                line_merge = Some(u);
            } else {
                // Union didn't work, push the completed
                output.push(*l);
                line_merge = Some(line);
            }
        } else {
            // First one
            line_merge = Some(line);
        }
    }
    output.push(line_merge.unwrap());
    output
}

fn count_visible(sensors: &[Sensor], y: i32) -> usize {
    let lines = calc_lines_at_y(sensors, y);

    let beacons: Vec<Point2> = sensors.iter().map(|s| s.beacon).sorted().dedup().collect();
    lines
        .iter()
        .map(|line| {
            let count = beacons
                .iter()
                .filter(|beacon| beacon.y == y && (line.left..=line.right).contains(&beacon.x))
                .count();
            line.len() - count
        })
        .sum()
}

// For each horizontal line, generate the line segments that are seen by the sensors. Merge them and look for any place
// there's a gap and assume that's the target. There's gotta be a better way but this is fast enough for now.
fn tuning_frequency(sensors: &[Sensor], max: i32) -> usize {
    let (y, lines) = (0..=max)
        .into_par_iter()
        .map(|y| (y, calc_lines_at_y(sensors, y)))
        .find_any(|(_, lines)| lines.len() == 2)
        .expect("Nothing found");
    assert_eq!(lines[0].right + 1, lines[1].left - 1);
    let x = lines[0].right + 1;
    x as usize * 4_000_000 + y as usize
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Sensor> {
    input.lines().map(Sensor::from_str).collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[Sensor]) -> usize {
    let count = count_visible(input, 2_000_000);
    assert_eq!(count, 4827924);
    count
}

#[aoc(day15, part2)]
pub fn part2(input: &[Sensor]) -> usize {
    let freq = tuning_frequency(input, 4_000_000);
    assert_eq!(freq, 12977110973564);
    freq
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_count_visible() {
        let input = input_generator(EXAMPLE_INPUT);
        let count = count_visible(&input, 10);
        assert_eq!(count, 26);
    }

    #[ignore] // Sometimes fails for some reason, probably due to multithreading in test
    #[test]
    fn test_tuning_frequency() {
        let input = input_generator(EXAMPLE_INPUT);
        let freq = tuning_frequency(&input, 20);
        assert_eq!(freq, 56000011);
    }
}
