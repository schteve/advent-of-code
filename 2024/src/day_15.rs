/*
    --- Day 15: Warehouse Woes ---
    You appear back inside your own mini submarine! Each Historian drives their mini submarine in a different direction; maybe the Chief has his own submarine down here somewhere as well?

    You look up to see a vast school of lanternfish swimming past you. On closer inspection, they seem quite anxious, so you drive your mini submarine over to see if you can help.

    Because lanternfish populations grow rapidly, they need a lot of food, and that food needs to be stored somewhere. That's why these lanternfish have built elaborate warehouse complexes operated by robots!

    These lanternfish seem so anxious because they have lost control of the robot that operates one of their most important warehouses! It is currently running amok, pushing around boxes in the warehouse with no regard for lanternfish logistics or lanternfish inventory management strategies.

    Right now, none of the lanternfish are brave enough to swim up to an unpredictable robot so they could shut it off. However, if you could anticipate the robot's movements, maybe they could find a safe option.

    The lanternfish already have a map of the warehouse and a list of movements the robot will attempt to make (your puzzle input). The problem is that the movements will sometimes fail as boxes are shifted around, making the actual movements of the robot difficult to predict.

    For example:

    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    As the robot (@) attempts to move, if there are any boxes (O) in the way, the robot will also attempt to push those boxes. However, if this action would cause the robot or a box to move into a wall (#), nothing moves instead, including the robot. The initial positions of these are shown on the map at the top of the document the lanternfish gave you.

    The rest of the document describes the moves (^ for up, v for down, < for left, > for right) that the robot will attempt to make, in order. (The moves form a single giant sequence; they are broken into multiple lines just to make copy-pasting easier. Newlines within the move sequence should be ignored.)

    Here is a smaller example to get started:

    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    <^^>>>vv<v>>v<<
    Were the robot to attempt the given sequence of moves, it would push around the boxes as follows:

    Initial state:
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move <:
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move ^:
    ########
    #.@O.O.#
    ##..O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move ^:
    ########
    #.@O.O.#
    ##..O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move >:
    ########
    #..@OO.#
    ##..O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move >:
    ########
    #...@OO#
    ##..O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move >:
    ########
    #...@OO#
    ##..O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    Move v:
    ########
    #....OO#
    ##..@..#
    #...O..#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move v:
    ########
    #....OO#
    ##..@..#
    #...O..#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move <:
    ########
    #....OO#
    ##.@...#
    #...O..#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move v:
    ########
    #....OO#
    ##.....#
    #..@O..#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move >:
    ########
    #....OO#
    ##.....#
    #...@O.#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move >:
    ########
    #....OO#
    ##.....#
    #....@O#
    #.#.O..#
    #...O..#
    #...O..#
    ########

    Move v:
    ########
    #....OO#
    ##.....#
    #.....O#
    #.#.O@.#
    #...O..#
    #...O..#
    ########

    Move <:
    ########
    #....OO#
    ##.....#
    #.....O#
    #.#O@..#
    #...O..#
    #...O..#
    ########

    Move <:
    ########
    #....OO#
    ##.....#
    #.....O#
    #.#O@..#
    #...O..#
    #...O..#
    ########
    The larger example has many more moves; after the robot has finished those moves, the warehouse would look like this:

    ##########
    #.O.O.OOO#
    #........#
    #OO......#
    #OO@.....#
    #O#.....O#
    #O.....OO#
    #O.....OO#
    #OO....OO#
    ##########
    The lanternfish use their own custom Goods Positioning System (GPS for short) to track the locations of the boxes. The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance from the left edge of the map. (This process does not stop at wall tiles; measure all the way to the edges of the map.)

    So, the box shown below has a distance of 1 from the top edge of the map and 4 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 4 = 104.

    #######
    #...O..
    #......
    The lanternfish would like to know the sum of all boxes' GPS coordinates after the robot finishes moving. In the larger example, the sum of all boxes' GPS coordinates is 10092. In the smaller example, the sum is 2028.

    Predict the motion of the robot and boxes in the warehouse. After the robot is finished moving, what is the sum of all boxes' GPS coordinates?

    --- Part Two ---
    The lanternfish use your information to find a safe moment to swim in and turn off the malfunctioning robot! Just as they start preparing a festival in your honor, reports start coming in that a second warehouse's robot is also malfunctioning.

    This warehouse's layout is surprisingly similar to the one you just helped. There is one key difference: everything except the robot is twice as wide! The robot's list of movements doesn't change.

    To get the wider warehouse's map, start with your original map and, for each tile, make the following changes:

    If the tile is #, the new map contains ## instead.
    If the tile is O, the new map contains [] instead.
    If the tile is ., the new map contains .. instead.
    If the tile is @, the new map contains @. instead.
    This will produce a new warehouse map which is twice as wide and with wide boxes that are represented by []. (The robot does not change size.)

    The larger example from before would now look like this:

    ####################
    ##....[]....[]..[]##
    ##............[]..##
    ##..[][]....[]..[]##
    ##....[]@.....[]..##
    ##[]##....[]......##
    ##[]....[]....[]..##
    ##..[][]..[]..[][]##
    ##........[]......##
    ####################
    Because boxes are now twice as wide but the robot is still the same size and speed, boxes can be aligned such that they directly push two other boxes at once. For example, consider this situation:

    #######
    #...#.#
    #.....#
    #..OO@#
    #..O..#
    #.....#
    #######

    <vv<<^^<<^^
    After appropriately resizing this map, the robot would push around these boxes as follows:

    Initial state:
    ##############
    ##......##..##
    ##..........##
    ##....[][]@.##
    ##....[]....##
    ##..........##
    ##############

    Move <:
    ##############
    ##......##..##
    ##..........##
    ##...[][]@..##
    ##....[]....##
    ##..........##
    ##############

    Move v:
    ##############
    ##......##..##
    ##..........##
    ##...[][]...##
    ##....[].@..##
    ##..........##
    ##############

    Move v:
    ##############
    ##......##..##
    ##..........##
    ##...[][]...##
    ##....[]....##
    ##.......@..##
    ##############

    Move <:
    ##############
    ##......##..##
    ##..........##
    ##...[][]...##
    ##....[]....##
    ##......@...##
    ##############

    Move <:
    ##############
    ##......##..##
    ##..........##
    ##...[][]...##
    ##....[]....##
    ##.....@....##
    ##############

    Move ^:
    ##############
    ##......##..##
    ##...[][]...##
    ##....[]....##
    ##.....@....##
    ##..........##
    ##############

    Move ^:
    ##############
    ##......##..##
    ##...[][]...##
    ##....[]....##
    ##.....@....##
    ##..........##
    ##############

    Move <:
    ##############
    ##......##..##
    ##...[][]...##
    ##....[]....##
    ##....@.....##
    ##..........##
    ##############

    Move <:
    ##############
    ##......##..##
    ##...[][]...##
    ##....[]....##
    ##...@......##
    ##..........##
    ##############

    Move ^:
    ##############
    ##......##..##
    ##...[][]...##
    ##...@[]....##
    ##..........##
    ##..........##
    ##############

    Move ^:
    ##############
    ##...[].##..##
    ##...@.[]...##
    ##....[]....##
    ##..........##
    ##..........##
    ##############
    This warehouse also uses GPS to locate the boxes. For these larger boxes, distances are measured from the edge of the map to the closest edge of the box in question. So, the box shown below has a distance of 1 from the top edge of the map and 5 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 5 = 105.

    ##########
    ##...[]...
    ##........
    In the scaled-up version of the larger example from above, after the robot has finished all of its moves, the warehouse would look like this:

    ####################
    ##[].......[].[][]##
    ##[]...........[].##
    ##[]........[][][]##
    ##[]......[]....[]##
    ##..##......[]....##
    ##..[]............##
    ##..@......[].[][]##
    ##......[][]..[]..##
    ####################
    The sum of these boxes' GPS coordinates is 9021.

    Predict the motion of the robot and boxes in this new, scaled-up warehouse. What is the sum of all boxes' final GPS coordinates?
*/

use std::collections::HashSet;

use common::{Cardinal, Point2, TileChar, TileMap};

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
            Self::Robot => '@',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => return None,
        })
    }

    fn all_chars() -> Vec<char> {
        vec!['#', 'O', '[', ']', '@']
    }
}

pub struct Warehouse {
    tiles: TileMap<Tile>,
    moves: Vec<Cardinal>,
}

impl Warehouse {
    fn parse(input: &str) -> Self {
        let (tiles_str, moves_str) = input.split_once("\n\n").unwrap();
        let tiles = TileMap::from_string(tiles_str);
        let moves = moves_str
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Cardinal::North),
                '>' => Some(Cardinal::East),
                'v' => Some(Cardinal::South),
                '<' => Some(Cardinal::West),
                _ => None,
            })
            .collect();

        Self { tiles, moves }
    }

    fn predict(&self) -> u64 {
        let mut tiles = self.tiles.clone();
        let mut robot = tiles
            .iter()
            .find_map(|(p, t)| (t == &Tile::Robot).then_some(*p))
            .unwrap();
        tiles.remove(&robot);

        for dir in &self.moves {
            let next = robot.step(*dir, 1);
            let mut end = next;
            while tiles.get(&end) == Some(&Tile::Box) {
                end = end.step(*dir, 1);
            }

            match tiles.get(&end) {
                Some(Tile::Wall) => (), // Nothing moves
                Some(Tile::Box) => panic!("Invalid: end can't be a box"),
                Some(Tile::BoxLeft) => panic!("Invalid: end can't be a box (left)"),
                Some(Tile::BoxRight) => panic!("Invalid: end can't be a box (right)"),
                Some(Tile::Robot) => panic!("Invalid: end can't be a robot"),
                None => {
                    // Move robot
                    tiles.remove(&next);
                    robot = next;

                    // Move boxes, if any
                    if next != end {
                        tiles.insert(end, Tile::Box);
                    }
                }
            }
        }

        //println!("{}", tiles);

        // Calculate score
        tiles
            .iter()
            .filter_map(|(p, t)| (t == &Tile::Box).then_some(p.x as u64 + p.y as u64 * 100))
            .sum()
    }

    fn predict_wide(&self) -> u64 {
        // Double it
        let mut tiles = TileMap::<Tile>::new();
        for (p, t) in self.tiles.iter() {
            match t {
                Tile::Wall => {
                    let new_p = Point2 { x: p.x * 2, y: p.y };
                    tiles.insert(new_p, Tile::Wall);
                    tiles.insert(new_p + (1, 0), Tile::Wall);
                }
                Tile::Box => {
                    let new_p = Point2 { x: p.x * 2, y: p.y };
                    tiles.insert(new_p, Tile::BoxLeft);
                    tiles.insert(new_p + (1, 0), Tile::BoxRight);
                }
                Tile::BoxLeft => panic!("Invalid big box (left)"),
                Tile::BoxRight => panic!("Invalid big box (right)"),
                Tile::Robot => {
                    let new_p = Point2 { x: p.x * 2, y: p.y };
                    tiles.insert(new_p, Tile::Robot);
                }
            }
        }

        // Setup
        let mut robot = tiles
            .iter()
            .find_map(|(p, t)| (t == &Tile::Robot).then_some(*p))
            .unwrap();
        tiles.remove(&robot);

        for dir in &self.moves {
            let next = robot.step(*dir, 1);

            let mut boxes_to_move = HashSet::new();
            let mut frontier = vec![next];
            let mut can_move = true;
            'outer: while !frontier.is_empty() {
                let mut tmp = Vec::new();
                for p in frontier.drain(..) {
                    match tiles.get(&p) {
                        Some(Tile::Wall) => {
                            can_move = false;
                            break 'outer;
                        }
                        Some(Tile::Box) => panic!("Invalid: can't be box in wide"),
                        Some(Tile::BoxLeft) => {
                            let left = p;
                            let right = p + (1, 0);
                            let left_next = left.step(*dir, 1);
                            let right_next = right.step(*dir, 1);

                            if *dir != Cardinal::East {
                                tmp.push(left_next);
                            }
                            tmp.push(right_next);
                            boxes_to_move.insert(left);
                            boxes_to_move.insert(right);
                        }
                        Some(Tile::BoxRight) => {
                            let left = p + (-1, 0);
                            let right = p;
                            let left_next = left.step(*dir, 1);
                            let right_next = right.step(*dir, 1);

                            tmp.push(left_next);
                            if *dir != Cardinal::West {
                                tmp.push(right_next);
                            }
                            boxes_to_move.insert(left);
                            boxes_to_move.insert(right);
                        }
                        Some(Tile::Robot) => panic!("Invalid: can't be robot"),
                        None => (),
                    }
                }

                frontier.extend(tmp.into_iter());
            }

            if can_move {
                // Move all boxes
                let tiles_to_move: Vec<_> = boxes_to_move
                    .into_iter()
                    .map(|b| (b, tiles.remove(&b).unwrap()))
                    .collect();
                tiles.extend(tiles_to_move.into_iter().map(|(b, t)| (b.step(*dir, 1), t)));

                // Move robot
                tiles.remove(&robot);
                robot = next;
            }
        }

        //println!("{}", tiles);

        // Calculate score
        tiles
            .iter()
            .filter_map(|(p, t)| (t == &Tile::BoxLeft).then_some(p.x as u64 + p.y as u64 * 100))
            .sum()
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Warehouse {
    Warehouse::parse(input)
}

#[aoc(day15, part1)]
pub fn part1(input: &Warehouse) -> u64 {
    let value = input.predict();
    assert_eq!(value, 1514333);
    value
}

#[aoc(day15, part2)]
pub fn part2(input: &Warehouse) -> u64 {
    let value = input.predict_wide();
    assert_eq!(value, 1528453);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    static EXAMPLE_INPUT_2: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_predict() {
        let test_data = [(EXAMPLE_INPUT_1, 10092), (EXAMPLE_INPUT_2, 2028)];
        for (input, expected) in test_data {
            let input = input_generator(input);
            let value = input.predict();
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn test_predict_wide() {
        let input = input_generator(EXAMPLE_INPUT_1);
        let value = input.predict_wide();
        assert_eq!(value, 9021);
    }
}
