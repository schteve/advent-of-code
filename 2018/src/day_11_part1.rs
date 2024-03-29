/*
    --- Day 11: Chronal Charge ---
    You watch the Elves and their sleigh fade into the distance as they head toward the North Pole.

    Actually, you're the one fading. The falling sensation returns.

    The low fuel warning light is illuminated on your wrist-mounted device. Tapping it once causes it to project a hologram of the situation: a 300x300 grid of fuel cells and their current power levels, some negative. You're not sure what negative power means in the context of time travel, but it can't be good.

    Each fuel cell has a coordinate ranging from 1 to 300 in both the X (horizontal) and Y (vertical) direction. In X,Y notation, the top-left cell is 1,1, and the top-right cell is 300,1.

    The interface lets you select any 3x3 square of fuel cells. To increase your chances of getting to your destination, you decide to choose the 3x3 square with the largest total power.

    The power level in a given fuel cell can be found through the following process:

    Find the fuel cell's rack ID, which is its X coordinate plus 10.
    Begin with a power level of the rack ID times the Y coordinate.
    Increase the power level by the value of the grid serial number (your puzzle input).
    Set the power level to itself multiplied by the rack ID.
    Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    Subtract 5 from the power level.
    For example, to find the power level of the fuel cell at 3,5 in a grid with serial number 8:

    The rack ID is 3 + 10 = 13.
    The power level starts at 13 * 5 = 65.
    Adding the serial number produces 65 + 8 = 73.
    Multiplying by the rack ID produces 73 * 13 = 949.
    The hundreds digit of 949 is 9.
    Subtracting 5 produces 9 - 5 = 4.
    So, the power level of this fuel cell is 4.

    Here are some more example power levels:

    Fuel cell at  122,79, grid serial number 57: power level -5.
    Fuel cell at 217,196, grid serial number 39: power level  0.
    Fuel cell at 101,153, grid serial number 71: power level  4.
    Your goal is to find the 3x3 square which has the largest total power. The square must be entirely within the 300x300 grid. Identify this square using the X,Y coordinate of its top-left fuel cell. For example:

    For grid serial number 18, the largest total 3x3 square has a top-left corner of 33,45 (with a total power of 29); these fuel cells appear in the middle of this 5x5 region:

    -2  -4   4   4   4
    -4   4   4   4  -5
    4   3   3   4  -4
    1   1   2   4  -3
    -1   0   2  -5  -2
    For grid serial number 42, the largest 3x3 square's top-left is 21,61 (with a total power of 30); they are in the middle of this region:

    -3   4   2   2   2
    -4   4   3   3   4
    -5   3   3   4  -4
    4   3   3   4  -3
    3   3   3  -5  -1
    What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?
*/

use common::Point2;
use std::collections::HashMap;

struct Grid {
    power: HashMap<Point2, i32>,
    size: u32,
}

impl Grid {
    fn new(size: u32) -> Self {
        Self {
            power: HashMap::new(),
            size,
        }
    }

    fn power_level(p: Point2, serial: i32) -> i32 {
        let rack_id = p.x + 10;
        let mut power = rack_id * p.y;
        power += serial;
        power *= rack_id;
        power = (power % 1000) / 100;
        power -= 5;
        power
    }

    fn fuel_grid(&mut self, serial: i32) {
        for x in 0..self.size {
            for y in 0..self.size {
                let p = Point2 {
                    x: x as i32,
                    y: y as i32,
                };
                let power = Grid::power_level(p, serial);
                self.power.insert(p, power);
            }
        }
    }

    fn total_power(&self, p: Point2) -> i32 {
        (0..3)
            .flat_map(|x| {
                (0..3).map(move |y| Point2 {
                    x: p.x + x,
                    y: p.y + y,
                })
            })
            .map(|p| self.power[&p])
            .sum()
    }

    fn max_power_point(&self) -> Point2 {
        (0..(self.size - 2))
            .flat_map(|x| {
                (0..(self.size - 2)).map(move |y| Point2 {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .max_by_key(|&p| self.total_power(p))
            .unwrap()
    }
}

#[aoc(day11, part1)]
pub fn solve(input: &str) -> Point2 {
    let serial = input.trim().parse::<i32>().unwrap();

    let mut grid = Grid::new(300);
    grid.fuel_grid(serial);

    let max_power_point = grid.max_power_point();
    println!("Largest total power: {}", max_power_point);
    assert_eq!(max_power_point, Point2 { x: 243, y: 68 });
    max_power_point
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        let point = Point2 { x: 3, y: 5 };
        let serial = 8;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 4);

        let point = Point2 { x: 122, y: 79 };
        let serial = 57;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, -5);

        let point = Point2 { x: 217, y: 196 };
        let serial = 39;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 0);

        let point = Point2 { x: 101, y: 153 };
        let serial = 71;
        let power = Grid::power_level(point, serial);
        assert_eq!(power, 4);
    }
}
