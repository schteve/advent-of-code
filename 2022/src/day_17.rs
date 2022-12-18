/*
    --- Day 17: Pyroclastic Flow ---
    Your handheld device has located an alternative exit from the cave for you and the elephants. The ground is rumbling almost continuously now, but the strange valves bought you some time. It's definitely getting warmer in here, though.

    The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped rocks are falling into the chamber from above, presumably due to all the rumbling. If you can't work out where the rocks will fall next, you might be crushed!

    The five types of rocks have the following peculiar shapes, where # is rock and . is empty space:

    ####

    .#.
    ###
    .#.

    ..#
    ..#
    ###

    #
    #
    #
    #

    ##
    ##
    The rocks fall in the order shown above: first the - shape, then the + shape, and so on. Once the end of the list is reached, the same order repeats: the - shape falls first, sixth, 11th, 16th, etc.

    The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls themselves. A quick scan reveals the effect the jets of hot gas will have on the rocks as they fall (your puzzle input).

    For example, suppose this was the jet pattern in your cave:

    >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    In jet patterns, < means a push to the left, while > means a push to the right. The pattern above means that the jets will push a falling rock right, then right, then right, then left, then left, then right, and so on. If the end of the list is reached, it repeats.

    The tall, vertical chamber is exactly seven units wide. Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).

    After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the direction indicated by the next symbol in the jet pattern) and then falling one unit down. If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock, the movement instead does not occur. If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock, the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.

    Drawing falling rocks with @ and stopped rocks with #, the jet pattern in the example above manifests as follows:

    The first rock begins falling:
    |..@@@@.|
    |.......|
    |.......|
    |.......|
    +-------+

    Jet of gas pushes rock right:
    |...@@@@|
    |.......|
    |.......|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    |.......|
    |.......|
    +-------+

    Jet of gas pushes rock right, but nothing happens:
    |...@@@@|
    |.......|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    |.......|
    +-------+

    Jet of gas pushes rock right, but nothing happens:
    |...@@@@|
    |.......|
    +-------+

    Rock falls 1 unit:
    |...@@@@|
    +-------+

    Jet of gas pushes rock left:
    |..@@@@.|
    +-------+

    Rock falls 1 unit, causing it to come to rest:
    |..####.|
    +-------+

    A new rock begins falling:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock left:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |.......|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock right:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |..####.|
    +-------+

    Jet of gas pushes rock left:
    |..@....|
    |.@@@...|
    |..@....|
    |.......|
    |..####.|
    +-------+

    Rock falls 1 unit:
    |..@....|
    |.@@@...|
    |..@....|
    |..####.|
    +-------+

    Jet of gas pushes rock right:
    |...@...|
    |..@@@..|
    |...@...|
    |..####.|
    +-------+

    Rock falls 1 unit, causing it to come to rest:
    |...#...|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    A new rock begins falling:
    |....@..|
    |....@..|
    |..@@@..|
    |.......|
    |.......|
    |.......|
    |...#...|
    |..###..|
    |...#...|
    |..####.|
    +-------+
    The moment each of the next few rocks begins falling, you would see this:

    |..@....|
    |..@....|
    |..@....|
    |..@....|
    |.......|
    |.......|
    |.......|
    |..#....|
    |..#....|
    |####...|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@...|
    |..@@...|
    |.......|
    |.......|
    |.......|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@@@.|
    |.......|
    |.......|
    |.......|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |...@...|
    |..@@@..|
    |...@...|
    |.......|
    |.......|
    |.......|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |....@..|
    |....@..|
    |..@@@..|
    |.......|
    |.......|
    |.......|
    |..#....|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@....|
    |..@....|
    |..@....|
    |..@....|
    |.......|
    |.......|
    |.......|
    |.....#.|
    |.....#.|
    |..####.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@...|
    |..@@...|
    |.......|
    |.......|
    |.......|
    |....#..|
    |....#..|
    |....##.|
    |....##.|
    |..####.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+

    |..@@@@.|
    |.......|
    |.......|
    |.......|
    |....#..|
    |....#..|
    |....##.|
    |##..##.|
    |######.|
    |.###...|
    |..#....|
    |.####..|
    |....##.|
    |....##.|
    |....#..|
    |..#.#..|
    |..#.#..|
    |#####..|
    |..###..|
    |...#...|
    |..####.|
    +-------+
    To prove to the elephants your simulation is accurate, they want to know how tall the tower will get after 2022 rocks have stopped (but before the 2023rd rock begins falling). In this example, the tower of rocks will be 3068 units tall.

    How many units tall will the tower of rocks be after 2022 rocks have stopped falling?

    --- Part Two ---
    The elephants are not impressed by your simulation. They demand to know how tall the tower will be after 1000000000000 rocks have stopped! Only then will they feel confident enough to proceed through the cave.

    In the example above, the tower would be 1514285714288 units tall!

    How tall will the tower be after 1000000000000 rocks have stopped?
*/

use std::fmt::Display;

// Bitwise pattern for the shape.
// Represent it as 8 bits wide (highest bit unused) and 4 rows tall so it matches the tower.
// Bitshift left is jet left. Bitshift right is jet right.
// Check bit 0 to see if can jet right, bit 6 to see if can jet left.
// Bitand with the tower's layers to check for collisions.
// Starting position for each piece is 2 from left.
#[derive(Clone, Copy)]
struct Shape(u32);

impl Shape {
    #[allow(clippy::unusual_byte_groupings)]
    fn is_on_right(self) -> bool {
        (self.0 & 0b00000001_00000001_00000001_00000001) != 0
    }

    #[allow(clippy::unusual_byte_groupings)]
    fn is_on_left(self) -> bool {
        (self.0 & 0b01000000_01000000_01000000_01000000) != 0
    }

    fn move_right(self) -> Self {
        if self.is_on_right() {
            self
        } else {
            Self(self.0 >> 1)
        }
    }

    fn move_left(self) -> Self {
        if self.is_on_left() {
            self
        } else {
            Self(self.0 << 1)
        }
    }

    fn row(self, row: usize) -> u8 {
        (self.0 >> (row * 8) & 0xFF) as u8
    }

    fn collides_with(self, tower: &[u8], overlap: i32) -> bool {
        if overlap <= 0 {
            return false;
        }

        let overlap_idx = tower.len() - overlap as usize;
        for (row_idx, row_val) in tower[overlap_idx..].iter().enumerate().take(4) {
            if row_val & self.row(row_idx) != 0 {
                return true;
            }
        }

        false
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..4).rev() {
            writeln!(f, "{}", print_row(self.row(row)))?;
        }
        Ok(())
    }
}

fn print_row(row: u8) -> String {
    (0..7)
        .rev()
        .map(|bit| if row & (1 << bit) == 0 { ' ' } else { '#' })
        .collect()
}

/*
    0 0000000
    0 0000000
    0 0000000
    0 0011110
*/
#[allow(clippy::unusual_byte_groupings)]
const HORIZ: Shape = Shape(0b0_0000000_0_0000000_0_0000000_0_0011110);

/*
    0 0000000
    0 0001000
    0 0011100
    0 0001000
*/
#[allow(clippy::unusual_byte_groupings)]
const PLUS: Shape = Shape(0b0_0000000_0_0001000_0_0011100_0_0001000);

/*
    0 0000000
    0 0000100
    0 0000100
    0 0011100
*/
#[allow(clippy::unusual_byte_groupings)]
const CORNER: Shape = Shape(0b0_0000000_0_0000100_0_0000100_0_0011100);

/*
    0 0010000
    0 0010000
    0 0010000
    0 0010000
*/
#[allow(clippy::unusual_byte_groupings)]
const VERT: Shape = Shape(0b0_0010000_0_0010000_0_0010000_0_0010000);

/*
    0 0000000
    0 0000000
    0 0011000
    0 0011000
*/
#[allow(clippy::unusual_byte_groupings)]
const SQUARE: Shape = Shape(0b0_0000000_0_0000000_0_0011000_0_0011000);

const SHAPES: [Shape; 5] = [HORIZ, PLUS, CORNER, VERT, SQUARE];

fn tetris(jets: &str) -> usize {
    let mut tower: Vec<u8> = vec![0x7F]; // Make things simple - the tower always starts with one solid row
    let mut jets_stream = jets.chars().cycle();

    for mut piece in SHAPES.into_iter().cycle().take(2022) {
        let mut overlap: i32 = -3; // Positive values indicate the piece overlaps the tower by some amount

        loop {
            let candidate = match jets_stream.next().unwrap() {
                '>' => piece.move_right(),
                '<' => piece.move_left(),
                x => panic!("Invalid jet {x}"),
            };

            let next_piece = if candidate.collides_with(&tower, overlap) {
                piece
            } else {
                candidate
            };

            // Try to move down
            if next_piece.collides_with(&tower, overlap + 1) {
                let overlap_idx = tower.len() - overlap as usize;
                // Merge the piece with any existing rows
                for row in (0..overlap as usize).take(4) {
                    tower[overlap_idx + row] |= next_piece.row(row);
                }
                // Add new rows
                for row in overlap as usize..4 {
                    let next_row = next_piece.row(row);
                    if next_row != 0 {
                        tower.push(next_row);
                    }
                }
                break;
            }

            // Can move down
            overlap += 1;
            piece = next_piece;
        }
    }

    tower.len() - 1
}

/*fn print_tower(tower: &[u8]) {
    println!("\nTower:");
    for &row in tower.iter().rev() {
        println!("{}", print_row(row));
    }
}*/

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let height = tetris(input);
    assert_eq!(height, 3130);
    height
}

/*#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    todo!()
    /*let x = x(input);
    assert_eq!(x, 123);
    x*/
}*/

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_tetris() {
        let height = tetris(EXAMPLE_INPUT);
        assert_eq!(height, 3068);
    }
}
