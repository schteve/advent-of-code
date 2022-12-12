/*
    --- Day 12: Hill Climbing Algorithm ---
    You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

    You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

    Also included on the heightmap are marks for your current position (S) and the location that should get the best signal (E). Your current position (S) has elevation a, and the location that should get the best signal (E) has elevation z.

    You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

    For example:

    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:

    v..v<<<<
    >v.vv<<^
    .>vv>E^^
    ..v>>>^^
    ..>>>>>^
    In the above diagram, the symbols indicate whether the path exits each square moving up (^), down (v), left (<), or right (>). The location that should get the best signal is still E, and . marks unvisited squares.

    This path reaches the goal in 31 steps, the fewest possible.

    What is the fewest steps required to move from your current position to the location that should get the best signal?

    --- Part Two ---
    As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't very scenic, though; perhaps you can find a better starting point.

    To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from any square at elevation a to the square marked E.

    Again consider the example from above:

    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

    ...v<<<<
    ...vv<<^
    ...v>E^^
    .>v>>>^^
    >^>>>>>^
    This path reaches the goal in only 29 steps, the fewest possible.

    What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?
*/

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use common::Point2;

pub struct Hill {
    start: Point2,
    end: Point2,
    map: HashMap<Point2, char>,
}

impl Hill {
    fn from_str(s: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point2::from((x as i32, y as i32));
                if start.is_none() && c == 'S' {
                    start = Some(p);
                }
                if end.is_none() && c == 'E' {
                    end = Some(p);
                }

                let elev = match c {
                    'a'..='z' => c,
                    'S' => 'a',
                    'E' => 'z',
                    _ => panic!("Invalid char: {c}"),
                };

                map.insert(p, elev);
            }
        }

        Self {
            start: start.unwrap(),
            end: end.unwrap(),
            map,
        }
    }

    fn steps_to_end(&self) -> usize {
        let mut frontier: Vec<(usize, Point2)> = vec![(0, self.start)];
        let mut visited: HashSet<Point2> = HashSet::new();
        visited.insert(self.start);
        while let Some((steps, curr)) = frontier.pop() {
            let curr_char = self.map[&curr];
            let next: Vec<_> = curr
                .orthogonals()
                .filter(|p| !visited.contains(p))
                .filter(|p| {
                    if let Some(c) = self.map.get(p) {
                        let diff = *c as i32 - curr_char as i32;
                        diff <= 1
                    } else {
                        false
                    }
                })
                .collect(); // There's gotta be a more elegant way to do this without allocating but checking `visited` fails the borrow checker

            if next.contains(&self.end) {
                return steps + 1;
            }

            visited.extend(next.iter());
            frontier.extend(next.iter().map(|p| (steps + 1, *p)));
            frontier.sort_unstable_by_key(|k| Reverse(k.0));
        }
        panic!("Couldn't find path to end");
    }

    fn steps_from_end(&self) -> usize {
        let mut frontier: Vec<(usize, Point2)> = vec![(0, self.end)];
        let mut visited: HashSet<Point2> = HashSet::new();
        visited.insert(self.end);
        while let Some((steps, curr)) = frontier.pop() {
            let curr_char = self.map[&curr];
            let next: Vec<_> = curr
                .orthogonals()
                .filter(|p| !visited.contains(p))
                .filter(|p| {
                    if let Some(c) = self.map.get(p) {
                        let diff = *c as i32 - curr_char as i32;
                        diff >= -1
                    } else {
                        false
                    }
                })
                .collect(); // There's gotta be a more elegant way to do this without allocating but checking `visited` fails the borrow checker

            if next
                .iter()
                .filter_map(|p| self.map.get(p))
                .any(|c| *c == 'a')
            {
                return steps + 1;
            }

            visited.extend(next.iter());
            frontier.extend(next.iter().map(|p| (steps + 1, *p)));
            frontier.sort_unstable_by_key(|k| Reverse(k.0));
        }
        panic!("Couldn't find path from end");
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Hill {
    Hill::from_str(input)
}

#[aoc(day12, part1)]
pub fn part1(input: &Hill) -> usize {
    let steps = input.steps_to_end();
    assert_eq!(steps, 520);
    steps
}

#[aoc(day12, part2)]
pub fn part2(input: &Hill) -> usize {
    let steps = input.steps_from_end();
    assert_eq!(steps, 508);
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_steps_to_end() {
        let input = input_generator(EXAMPLE_INPUT);
        let steps = input.steps_to_end();
        assert_eq!(steps, 31);
    }

    #[test]
    fn test_steps_from_end() {
        let input = input_generator(EXAMPLE_INPUT);
        let steps = input.steps_from_end();
        assert_eq!(steps, 29);
    }
}
