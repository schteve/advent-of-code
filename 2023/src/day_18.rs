/*

*/

use common::{Cardinal, Mode, Point2};

struct Instruction {
    dir: Cardinal,
    num: u32,
    true_dir: Cardinal,
    true_num: u32,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut splits = value.split_ascii_whitespace();
        let dir = match splits.next().unwrap() {
            "U" => Cardinal::North,
            "D" => Cardinal::South,
            "L" => Cardinal::West,
            "R" => Cardinal::East,
            x => panic!("Invalid dir: {x}"),
        };
        let num = splits.next().unwrap().parse().unwrap();

        let color = splits.next().unwrap();
        let color_trimmed = color
            .trim_start_matches('(')
            .trim_start_matches('#')
            .trim_end_matches(')');
        let true_num = u32::from_str_radix(&color_trimmed[0..5], 16).unwrap();
        let true_dir = match &color_trimmed[5..] {
            "0" => Cardinal::East,
            "1" => Cardinal::South,
            "2" => Cardinal::West,
            "3" => Cardinal::North,
            x => panic!("Invalid dir: {x}"),
        };

        Self {
            dir,
            num,
            true_dir,
            true_num,
        }
    }
}

pub struct DigPlan {
    instructions: Vec<Instruction>,
}

impl DigPlan {
    fn dig(&self, mode: Mode) -> u64 {
        // Shoelace formula
        let mut points = vec![Point2::origin()];
        let mut curr = Point2::origin();
        for inst in &self.instructions {
            let (dir, num) = if mode == Mode::M1 {
                (inst.dir, inst.num)
            } else {
                (inst.true_dir, inst.true_num)
            };
            curr = curr.step(dir, num as i32);
            points.push(curr);
        }

        let mut area = 0;
        for i in 0..self.instructions.len() {
            area += points[i].x as i64 * points[i + 1].y as i64
                - points[i + 1].x as i64 * points[i].y as i64;
        }
        area /= 2;

        // The shoelace area calculation is from the centers of the cubic holes, but we need the full volume.
        // Consider a dig like this:
        // ###
        // #.#
        // ###
        // The shoelace area is 4 because it looks like a 2x2 square. But we know the answer is 9.
        // If we zoom in, so each . is the center of the tile, we can see that the box drawn
        // from 0,0 to 0,2 to 2,2 to 2,0 actually leaves out a good chunk of the perimeter.
        // -------
        // |.|.|.|
        // -------
        // |.|.|.|
        // -------
        // |.|.|.|
        // -------
        // Actually, the missing area is half of the edge squares and 3/4 of the outside corners (and 1/4 of any inside
        // corners, but there's none in this example).
        // Since there should always be 4 more outside corners than inside corners, this works out to be an average of
        // half a square per perimeter tile, plus 4 * 1/4.
        // Thus after taking the area we need to add half of the perimeter plus 1.

        let perimeter: u64 = points
            .windows(2)
            .map(|window| Point2::manhattan(window[0], window[1]) as u64)
            .sum();

        area.unsigned_abs() + perimeter / 2 + 1
    }
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        let instructions = value.lines().map(Instruction::from).collect();
        Self { instructions }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> DigPlan {
    input.into()
}

#[aoc(day18, part1)]
pub fn part1(input: &DigPlan) -> u64 {
    let value = input.dig(Mode::M1);
    assert_eq!(value, 45159);
    value
}

#[aoc(day18, part2)]
pub fn part2(input: &DigPlan) -> u64 {
    let value = input.dig(Mode::M2);
    assert_eq!(value, 134549294799713);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_dig() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.dig(Mode::M1);
        assert_eq!(value, 62);
        let value = input.dig(Mode::M2);
        assert_eq!(value, 952408144115);
    }
}
