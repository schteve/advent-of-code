/*
    --- Day 12: Rain Risk ---
    Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

    Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

    The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.
    The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

    For example:

    F10
    N3
    F7
    R90
    F11
    These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
    F11 would move the ship 11 units south to east 17, south 8.
    At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

    Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

    --- Part Two ---
    Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

    Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

    Action N means to move the waypoint north by the given value.
    Action S means to move the waypoint south by the given value.
    Action E means to move the waypoint east by the given value.
    Action W means to move the waypoint west by the given value.
    Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    Action F means to move forward to the waypoint a number of times equal to the given value.
    The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

    For example, using the same instructions as above:

    F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
    N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
    F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
    R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
    F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
    After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

    Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
*/

use crate::common::{Cardinal, Point, Turn};
use nom::{
    character::complete::{digit1, multispace0, one_of},
    combinator::{map, map_res},
    multi::many1,
    sequence::{pair, preceded},
    IResult,
};

pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    fn parser(input: &str) -> IResult<&str, Self> {
        map(one_of("NSEWLRF"), |c: char| match c {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            'L' => Self::Left,
            'R' => Self::Right,
            'F' => Self::Forward,
            _ => panic!("Invalid action character received!"),
        })(input)
    }
}

pub struct Instruction {
    action: Action,
    value: u32,
}

impl Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (action, value)) = pair(
            preceded(multispace0, Action::parser),
            map_res(digit1, |d: &str| d.parse::<u32>()),
        )(input)?;

        Ok((input, Self { action, value }))
    }
}

struct Ship {
    location: Point,
    direction: Cardinal,
    waypoint: Point,
}

impl Ship {
    fn new() -> Self {
        Self {
            location: Point::new(),
            direction: Cardinal::East,
            waypoint: Point { x: 10, y: -1 },
        }
    }

    fn execute1(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => {
                self.location = self
                    .location
                    .step(Cardinal::North, instruction.value as i32)
            }
            Action::South => {
                self.location = self
                    .location
                    .step(Cardinal::South, instruction.value as i32)
            }
            Action::East => {
                self.location = self.location.step(Cardinal::East, instruction.value as i32)
            }
            Action::West => {
                self.location = self.location.step(Cardinal::West, instruction.value as i32)
            }
            Action::Left => {
                for _ in 0..instruction.value / 90 {
                    self.direction = self.direction.turn(Turn::Left);
                }
            }
            Action::Right => {
                for _ in 0..instruction.value / 90 {
                    self.direction = self.direction.turn(Turn::Right);
                }
            }
            Action::Forward => {
                self.location = self.location.step(self.direction, instruction.value as i32)
            }
        }
    }

    fn execute2(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => {
                self.waypoint = self
                    .waypoint
                    .step(Cardinal::North, instruction.value as i32)
            }
            Action::South => {
                self.waypoint = self
                    .waypoint
                    .step(Cardinal::South, instruction.value as i32)
            }
            Action::East => {
                self.waypoint = self.waypoint.step(Cardinal::East, instruction.value as i32)
            }
            Action::West => {
                self.waypoint = self.waypoint.step(Cardinal::West, instruction.value as i32)
            }
            Action::Left => {
                for _ in 0..instruction.value / 90 {
                    let tmp = self.waypoint.x;
                    self.waypoint.x = self.waypoint.y;
                    self.waypoint.y = -tmp;
                }
            }
            Action::Right => {
                for _ in 0..instruction.value / 90 {
                    let tmp = self.waypoint.x;
                    self.waypoint.x = -self.waypoint.y;
                    self.waypoint.y = tmp;
                }
            }
            Action::Forward => {
                for _ in 0..instruction.value {
                    self.location += self.waypoint;
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Mode {
    M1,
    M2,
}

fn handle_instructions(instructions: &[Instruction], mode: Mode) -> u32 {
    let mut ship = Ship::new();
    for instruction in instructions {
        match mode {
            Mode::M1 => ship.execute1(instruction),
            Mode::M2 => ship.execute2(instruction),
        }
    }

    Point::manhattan(Point::new(), ship.location)
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    many1(Instruction::parser)(input).unwrap().1
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> u32 {
    let distance = handle_instructions(input, Mode::M1);
    assert_eq!(distance, 2057);
    distance
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instruction]) -> u32 {
    let distance = handle_instructions(input, Mode::M2);
    assert_eq!(distance, 71504);
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_ship_execute1() {
        let mut ship = Ship::new();
        let mut instr_iter = input_generator(EXAMPLE_INPUT).into_iter();

        ship.execute1(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 10, y: 0 });
        assert_eq!(ship.direction, Cardinal::East);

        ship.execute1(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 10, y: -3 });
        assert_eq!(ship.direction, Cardinal::East);

        ship.execute1(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 17, y: -3 });
        assert_eq!(ship.direction, Cardinal::East);

        ship.execute1(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 17, y: -3 });
        assert_eq!(ship.direction, Cardinal::South);

        ship.execute1(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 17, y: 8 });
        assert_eq!(ship.direction, Cardinal::South);
    }

    #[test]
    fn test_ship_execute2() {
        let mut ship = Ship::new();
        let mut instr_iter = input_generator(EXAMPLE_INPUT).into_iter();

        ship.execute2(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 100, y: -10 });
        assert_eq!(ship.waypoint, Point { x: 10, y: -1 });

        ship.execute2(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 100, y: -10 });
        assert_eq!(ship.waypoint, Point { x: 10, y: -4 });

        ship.execute2(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 170, y: -38 });
        assert_eq!(ship.waypoint, Point { x: 10, y: -4 });

        ship.execute2(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 170, y: -38 });
        assert_eq!(ship.waypoint, Point { x: 4, y: 10 });

        ship.execute2(&instr_iter.next().unwrap());
        assert_eq!(ship.location, Point { x: 214, y: 72 });
        assert_eq!(ship.waypoint, Point { x: 4, y: 10 });
    }

    #[test]
    fn test_handle_instructions() {
        let instructions = input_generator(EXAMPLE_INPUT);
        let distance = handle_instructions(&instructions, Mode::M1);
        assert_eq!(distance, 25);

        let instructions = input_generator(EXAMPLE_INPUT);
        let distance = handle_instructions(&instructions, Mode::M2);
        assert_eq!(distance, 286);
    }
}
