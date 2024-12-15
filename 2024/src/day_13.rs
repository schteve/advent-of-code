/*
    --- Day 13: Claw Contraption ---
    Next up: the lobby of a resort on a tropical island. The Historians take a moment to admire the hexagonal floor tiles before spreading out.

    Fortunately, it looks like the resort has a new arcade! Maybe you can win some prizes from the claw machines?

    The claw machines here are a little unusual. Instead of a joystick or directional buttons to control the claw, these machines have two buttons labeled A and B. Worse, you can't just put in a token and play; it costs 3 tokens to push the A button and 1 token to push the B button.

    With a little experimentation, you figure out that each machine's buttons are configured to move the claw a specific amount to the right (along the X axis) and a specific amount forward (along the Y axis) each time that button is pressed.

    Each machine contains one prize; to win the prize, the claw must be positioned exactly above the prize on both the X and Y axes.

    You wonder: what is the smallest number of tokens you would have to spend to win as many prizes as possible? You assemble a list of every machine's button behavior and prize location (your puzzle input). For example:

    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
    This list describes the button configuration and prize location of four different claw machines.

    For now, consider just the first claw machine in the list:

    Pushing the machine's A button would move the claw 94 units along the X axis and 34 units along the Y axis.
    Pushing the B button would move the claw 22 units along the X axis and 67 units along the Y axis.
    The prize is located at X=8400, Y=5400; this means that from the claw's initial position, it would need to move exactly 8400 units along the X axis and exactly 5400 units along the Y axis to be perfectly aligned with the prize in this machine.
    The cheapest way to win the prize is by pushing the A button 80 times and the B button 40 times. This would line up the claw along the X axis (because 80*94 + 40*22 = 8400) and along the Y axis (because 80*34 + 40*67 = 5400). Doing this would cost 80*3 tokens for the A presses and 40*1 for the B presses, a total of 280 tokens.

    For the second and fourth claw machines, there is no combination of A and B presses that will ever win a prize.

    For the third claw machine, the cheapest way to win the prize is by pushing the A button 38 times and the B button 86 times. Doing this would cost a total of 200 tokens.

    So, the most prizes you could possibly win is two; the minimum tokens you would have to spend to win all (two) prizes is 480.

    You estimate that each button would need to be pressed no more than 100 times to win a prize. How else would someone be expected to play?

    Figure out how to win as many prizes as possible. What is the fewest tokens you would have to spend to win all possible prizes?

    --- Part Two ---
    As you go to win the first prize, you discover that the claw is nowhere near where you expected it would be. Due to a unit conversion error in your measurements, the position of every prize is actually 10000000000000 higher on both the X and Y axis!

    Add 10000000000000 to the X and Y position of every prize. After making this change, the example above would now look like this:

    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=10000000008400, Y=10000000005400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=10000000012748, Y=10000000012176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=10000000007870, Y=10000000006450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=10000000018641, Y=10000000010279
    Now, it is only possible to win a prize on the second and fourth claw machines. Unfortunately, it will take many more than 100 presses to do so.

    Using the corrected prize coordinates, figure out how to win as many prizes as possible. What is the fewest tokens you would have to spend to win all possible prizes?
*/

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub},
};

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point2 {
    x: i64,
    y: i64,
}

impl Add<Point2> for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Point2> for Point2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Point2> for Point2 {
    type Output = Self;

    fn sub(self, other: Point2) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i64> for Point2 {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        Point2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

pub struct Machine {
    a: Point2,
    b: Point2,
    prize: Point2,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        let (x, y) = lines[0]
            .trim_start_matches("Button A: ")
            .split_once(", ")
            .unwrap();
        let x = x.trim_start_matches('X').parse::<i64>().unwrap();
        let y = y.trim_start_matches('Y').parse::<i64>().unwrap();
        let a = Point2 { x, y };

        let (x, y) = lines[1]
            .trim_start_matches("Button B: ")
            .split_once(", ")
            .unwrap();
        let x = x.trim_start_matches('X').parse::<i64>().unwrap();
        let y = y.trim_start_matches('Y').parse::<i64>().unwrap();
        let b = Point2 { x, y };

        let (x, y) = lines[2]
            .trim_start_matches("Prize: ")
            .split_once(", ")
            .unwrap();
        let x = x.trim_start_matches("X=").parse::<i64>().unwrap();
        let y = y.trim_start_matches("Y=").parse::<i64>().unwrap();
        let prize = Point2 { x, y };

        Self { a, b, prize }
    }

    fn win(&self, offset: Point2) -> Option<u64> {
        let prize = self.prize + offset;

        let a_dydx = self.a.y as f32 / self.a.x as f32;
        let b_dydx = self.b.y as f32 / self.b.x as f32;
        let (a, b, a_value, b_value) = if a_dydx > b_dydx {
            (self.a, self.b, 3, 1)
        } else {
            (self.b, self.a, 1, 3)
        };

        let mut low = 0;
        let mut high = 10000000000000; // Arbitrary, probably big enough

        while low < high {
            let a_presses = (low + high) / 2;
            let a_part = a * a_presses;
            let b_presses = (prize.x - a_part.x) / b.x;
            let b_part = b * b_presses;
            let ab = a_part + b_part;
            if ab == prize {
                let tokens = a_value * a_presses + b_value * b_presses;
                return Some(tokens.try_into().unwrap());
            } else {
                // Need to account for the fraction when determining over/under
                let b_presses_f = (prize.x - a_part.x) as f64 / b.x as f64;
                let b_part_y = b.y as f64 * b_presses_f;
                let diff = prize.y as f64 - (a_part.y as f64 + b_part_y);
                if diff > 0.0 {
                    // Not enough A so go higher
                    low = a_presses + 1;
                } else {
                    // Too much A so go lower
                    high = a_presses;
                }
            }
        }

        None
    }
}

pub struct Arcade {
    machines: Vec<Machine>,
}

impl Arcade {
    fn parse(input: &str) -> Self {
        let machines = input.split("\n\n").map(Machine::parse).collect();
        Self { machines }
    }

    fn win(&self, offset: Point2) -> u64 {
        self.machines
            .iter()
            .filter_map(|machine| machine.win(offset))
            .sum()
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Arcade {
    Arcade::parse(input)
}

#[aoc(day13, part1)]
pub fn part1(input: &Arcade) -> u64 {
    let value = input.win(Point2 { x: 0, y: 0 });
    assert_eq!(value, 36954);
    value
}

#[aoc(day13, part2)]
pub fn part2(input: &Arcade) -> u64 {
    let value = input.win(Point2 {
        x: 10000000000000,
        y: 10000000000000,
    });
    assert_eq!(value, 79352015273424);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_win() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.win(Point2 { x: 0, y: 0 });
        assert_eq!(value, 480);

        let input = input_generator(EXAMPLE_INPUT);
        let value = input.win(Point2 {
            x: 10000000000000,
            y: 10000000000000,
        });
        assert_eq!(value, 875318608908);
    }
}
