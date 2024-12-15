/*
    --- Day 14: Restroom Redoubt ---
    One of The Historians needs to use the bathroom; fortunately, you know there's a bathroom near an unvisited location on their list, and so you're all quickly teleported directly to the lobby of Easter Bunny Headquarters.

    Unfortunately, EBHQ seems to have "improved" bathroom security again after your last visit. The area outside the bathroom is swarming with robots!

    To get The Historian safely to the bathroom, you'll need a way to predict where the robots will be in the future. Fortunately, they all seem to be moving on the tile floor in predictable straight lines.

    You make a list (your puzzle input) of all of the robots' current positions (p) and velocities (v), one robot per line. For example:

    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    Each robot's position is given as p=x,y where x represents the number of tiles the robot is from the left wall and y represents the number of tiles from the top wall (when viewed from above). So, a position of p=0,0 means the robot is all the way in the top-left corner.

    Each robot's velocity is given as v=x,y where x and y are given in tiles per second. Positive x means the robot is moving to the right, and positive y means the robot is moving down. So, a velocity of v=1,-2 means that each second, the robot moves 1 tile to the right and 2 tiles up.

    The robots outside the actual bathroom are in a space which is 101 tiles wide and 103 tiles tall (when viewed from above). However, in this example, the robots are in a space which is only 11 tiles wide and 7 tiles tall.

    The robots are good at navigating over/under each other (due to a combination of springs, extendable legs, and quadcopters), so they can share the same tile and don't interact with each other. Visually, the number of robots on each tile in this example looks like this:

    1.12.......
    ...........
    ...........
    ......11.11
    1.1........
    .........1.
    .......1...
    These robots have a unique feature for maximum bathroom security: they can teleport. When a robot would run into an edge of the space they're in, they instead teleport to the other side, effectively wrapping around the edges. Here is what robot p=2,4 v=2,-3 does for the first few seconds:

    Initial state:
    ...........
    ...........
    ...........
    ...........
    ..1........
    ...........
    ...........

    After 1 second:
    ...........
    ....1......
    ...........
    ...........
    ...........
    ...........
    ...........

    After 2 seconds:
    ...........
    ...........
    ...........
    ...........
    ...........
    ......1....
    ...........

    After 3 seconds:
    ...........
    ...........
    ........1..
    ...........
    ...........
    ...........
    ...........

    After 4 seconds:
    ...........
    ...........
    ...........
    ...........
    ...........
    ...........
    ..........1

    After 5 seconds:
    ...........
    ...........
    ...........
    .1.........
    ...........
    ...........
    ...........
    The Historian can't wait much longer, so you don't have to simulate the robots for very long. Where will the robots be after 100 seconds?

    In the above example, the number of robots on each tile after 100 seconds has elapsed looks like this:

    ......2..1.
    ...........
    1..........
    .11........
    .....1.....
    ...12......
    .1....1....
    To determine the safest area, count the number of robots in each quadrant after 100 seconds. Robots that are exactly in the middle (horizontally or vertically) don't count as being in any quadrant, so the only relevant robots are:

    ..... 2..1.
    ..... .....
    1.... .....

    ..... .....
    ...12 .....
    .1... 1....
    In this example, the quadrants contain 1, 3, 4, and 1 robot. Multiplying these together gives a total safety factor of 12.

    Predict the motion of the robots in your list within a space which is 101 tiles wide and 103 tiles tall. What will the safety factor be after exactly 100 seconds have elapsed?

    --- Part Two ---
    During the bathroom break, someone notices that these robots seem awfully similar to ones built and used at the North Pole. If they're the same type of robots, they should have a hard-coded Easter egg: very rarely, most of the robots should arrange themselves into a picture of a Christmas tree.

    What is the fewest number of seconds that must elapse for the robots to display the Easter egg?


*/

use std::collections::HashSet;

use common::{modulo, Point2};

#[derive(Clone)]
pub struct Robot {
    pos: Point2,
    vel: Point2,
}

impl Robot {
    fn parse(input: &str) -> Self {
        let (pos_str, vel_str) = input.split_once(' ').unwrap();

        let (pos_x, pos_y) = pos_str.split_once('=').unwrap().1.split_once(',').unwrap();
        let pos = Point2 {
            x: pos_x.parse::<i32>().unwrap(),
            y: pos_y.parse::<i32>().unwrap(),
        };

        let (vel_x, vel_y) = vel_str.split_once('=').unwrap().1.split_once(',').unwrap();
        let vel = Point2 {
            x: vel_x.parse::<i32>().unwrap(),
            y: vel_y.parse::<i32>().unwrap(),
        };

        Self { pos, vel }
    }

    fn step(&mut self, max_x: i32, max_y: i32) {
        self.pos.x = modulo(self.pos.x + self.vel.x, max_x);
        self.pos.y = modulo(self.pos.y + self.vel.y, max_y);
    }
}

pub struct RobotMap {
    robots: Vec<Robot>,
}

impl RobotMap {
    fn parse(input: &str) -> Self {
        let robots = input.lines().map(Robot::parse).collect();

        Self { robots }
    }

    fn safety_factor(&self, steps: usize, max_x: i32, max_y: i32) -> u64 {
        let mut bots = self.robots.clone();
        for _ in 0..steps {
            for bot in &mut bots {
                bot.step(max_x, max_y);
            }
        }

        let ul = bots
            .iter()
            .filter(|bot| bot.pos.x < max_x / 2 && bot.pos.y < max_y / 2)
            .count();
        let ur = bots
            .iter()
            .filter(|bot| bot.pos.x > max_x / 2 && bot.pos.y < max_y / 2)
            .count();
        let bl = bots
            .iter()
            .filter(|bot| bot.pos.x < max_x / 2 && bot.pos.y > max_y / 2)
            .count();
        let br = bots
            .iter()
            .filter(|bot| bot.pos.x > max_x / 2 && bot.pos.y > max_y / 2)
            .count();
        (ul * ur * bl * br) as u64
    }

    fn easter_egg(&self, max_x: i32, max_y: i32) -> u64 {
        // Assume they converge around a single location and it happens when variance is minimized
        let mut bots = self.robots.clone();
        let mut var_min = i64::MAX;
        let mut var_step = 0;
        let mut var_bots = Vec::new();
        for i in 0..10_000 {
            let sum = bots.iter().fold(Point2::origin(), |acc, bot| acc + bot.pos);
            let avg = Point2 {
                x: sum.x / self.robots.len() as i32,
                y: sum.y / self.robots.len() as i32,
            };
            let variance = bots.iter().fold(0i64, |acc, bot| {
                let err = bot.pos - avg;
                let mag = err.x as i64 + err.y as i64;
                acc + mag * mag
            });
            if variance < var_min {
                var_min = variance;
                var_step = i;
                var_bots = bots.clone();
            }

            for bot in &mut bots {
                bot.step(max_x, max_y);
            }
        }

        let bot_set: HashSet<Point2> = HashSet::from_iter(var_bots.iter().map(|bot| bot.pos));
        let range = Point2::get_range(var_bots.iter().map(|bot| &bot.pos)).unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let p = Point2 { x, y };
                if bot_set.contains(&p) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        var_step
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> RobotMap {
    RobotMap::parse(input)
}

#[aoc(day14, part1)]
pub fn part1(input: &RobotMap) -> u64 {
    let value = input.safety_factor(100, 101, 103);
    assert_eq!(value, 211692000);
    value
}

#[aoc(day14, part2)]
pub fn part2(input: &RobotMap) -> u64 {
    let value = input.easter_egg(101, 103);
    assert_eq!(value, 6587);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_safety_factor() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.safety_factor(100, 11, 7);
        assert_eq!(value, 12);
    }
}
