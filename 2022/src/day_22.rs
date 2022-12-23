/*
    --- Day 22: Monkey Map ---
    The monkeys take you on a surprisingly easy trail through the jungle. They're even going in roughly the right direction according to your handheld device's Grove Positioning System.

    As you walk, the monkeys explain that the grove is protected by a force field. To pass through the force field, you have to enter a password; doing so involves tracing a specific path on a strangely-shaped board.

    At least, you're pretty sure that's what you have to do; the elephants aren't exactly fluent in monkey.

    The monkeys give you notes that they took when they last saw the password entered (your puzzle input).

    For example:

            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.

    10R5L5R10L4R5L5
    The first half of the monkeys' notes is a map of the board. It is comprised of a set of open tiles (on which you can move, drawn .) and solid walls (tiles which you cannot enter, drawn #).

    The second half is a description of the path you must follow. It consists of alternating numbers and letters:

    A number indicates the number of tiles to move in the direction you are facing. If you run into a wall, you stop moving forward and continue with the next instruction.
    A letter indicates whether to turn 90 degrees clockwise (R) or counterclockwise (L). Turning happens in-place; it does not change your current tile.
    So, a path like 10R5 means "go forward 10 tiles, then turn clockwise 90 degrees, then go forward 5 tiles".

    You begin the path in the leftmost open tile of the top row of tiles. Initially, you are facing to the right (from the perspective of how the map is drawn).

    If a movement instruction would take you off of the map, you wrap around to the other side of the board. In other words, if your next tile is off of the board, you should instead look in the direction opposite of your current facing as far as you can until you find the opposite edge of the board, then reappear there.

    For example, if you are at A and facing to the right, the tile in front of you is marked B; if you are at C and facing down, the tile in front of you is marked D:

            ...#
            .#..
            #...
            ....
    ...#.D.....#
    ........#...
    B.#....#...A
    .....C....#.
            ...#....
            .....#..
            .#......
            ......#.
    It is possible for the next tile (after wrapping around) to be a wall; this still counts as there being a wall in front of you, and so movement stops before you actually wrap to the other side of the board.

    By drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example looks like this:

            >>v#
            .#v.
            #.v.
            ..v.
    ...#...v..v#
    >>>v...>#.>>
    ..#v...#....
    ...>>>>v..#.
            ...#....
            .....#..
            .#......
            ......#.
    To finish providing the password to this strange input device, you need to determine numbers for your final row, column, and facing as your final position appears from the perspective of the original map. Rows start from 1 at the top and count downward; columns start from 1 at the left and count rightward. (In the above example, row 1, column 1 refers to the empty space with no tile on it in the top-left corner.) Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^). The final password is the sum of 1000 times the row, 4 times the column, and the facing.

    In the above example, the final row is 6, the final column is 8, and the final facing is 0. So, the final password is 1000 * 6 + 4 * 8 + 0: 6032.

    Follow the path given in the monkeys' notes. What is the final password?

    --- Part Two ---
    As you reach the force field, you think you hear some Elves in the distance. Perhaps they've already arrived?

    You approach the strange input device, but it isn't quite what the monkeys drew in their notes. Instead, you are met with a large cube; each of its six faces is a square of 50x50 tiles.

    To be fair, the monkeys' map does have six 50x50 regions on it. If you were to carefully fold the map, you should be able to shape it into a cube!

    In the example above, the six (smaller, 4x4) faces of the cube are:

            1111
            1111
            1111
            1111
    222233334444
    222233334444
    222233334444
    222233334444
            55556666
            55556666
            55556666
            55556666
    You still start in the same position and with the same facing as before, but the wrapping rules are different. Now, if you would walk off the board, you instead proceed around the cube. From the perspective of the map, this can look a little strange. In the above example, if you are at A and move to the right, you would arrive at B facing down; if you are at C and move down, you would arrive at D facing up:

            ...#
            .#..
            #...
            ....
    ...#.......#
    ........#..A
    ..#....#....
    .D........#.
            ...#..B.
            .....#..
            .#......
            ..C...#.
    Walls still block your path, even if they are on a different face of the cube. If you are at E facing up, your movement is blocked by the wall marked by the arrow:

            ...#
            .#..
        -->#...
            ....
    ...#..E....#
    ........#...
    ..#....#....
    ..........#.
            ...#....
            .....#..
            .#......
            ......#.
    Using the same method of drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example now looks like this:

            >>v#
            .#v.
            #.v.
            ..v.
    ...#..^...v#
    .>>>>>^.#.>>
    .^#....#....
    .^........#.
            ...#..v.
            .....#v.
            .#v<<<<.
            ..v...#.
    The final password is still calculated from your final position and facing from the perspective of the map. In this example, the final row is 5, the final column is 7, and the final facing is 3, so the final password is 1000 * 5 + 4 * 7 + 3 = 5031.

    Fold the map into a cube, then follow the path given in the monkeys' notes. What is the final password?
*/

use std::{collections::HashMap, iter};

use common::{Cardinal, Point2, TileChar, TileMap, Turn};
use nom::{branch::alt, character::complete::char, combinator::map, multi::many1, IResult};

pub enum Tile {
    Open,
    Solid,
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Open => '.',
            Tile::Solid => '#',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Open),
            '#' => Some(Tile::Solid),
            _ => None,
        }
    }

    fn all_chars() -> Vec<char> {
        ['.', '#'].to_vec()
    }
}

#[derive(Debug)]
pub enum Step {
    Forward(u32),
    TurnLeft,
    TurnRight,
}

impl Step {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, step) = common::trim_start(alt((
            map(common::unsigned::<u32>, Self::Forward),
            map(char('L'), |_| Self::TurnLeft),
            map(char('R'), |_| Self::TurnRight),
        )))(input)?;

        Ok((input, step))
    }
}

pub struct Board {
    map: TileMap<Tile, ' '>,
    path: Vec<Step>,
    start: Point2,
}

impl Board {
    fn from_str(s: &str) -> Self {
        let (map_str, path_str) = s.split_once("\n\n").unwrap();
        let map = TileMap::from_string(map_str);
        let path = many1(Step::parser)(path_str).unwrap().1;

        let range = map.get_range().unwrap();
        let start = (range.x.0..=range.x.1)
            .find_map(|x| {
                let p = Point2 { x, y: range.y.0 };
                if map.contains_key(&p) {
                    Some(p)
                } else {
                    None
                }
            })
            .unwrap();

        Self { map, path, start }
    }

    fn wrap_pairs_2d(&self) -> HashMap<(Point2, Point2), (Point2, Cardinal)> {
        let mut wrap_pairs: HashMap<(Point2, Point2), (Point2, Cardinal)> = HashMap::new();
        let bounds = self.map.get_range().unwrap();

        // Wrap rows
        for y in bounds.y.0 ..= bounds.y.1 {
            let mut x_first = None;
            let mut x_last = None;
            for x in bounds.x.0-1 ..= bounds.x.1+1 {
                let p = Point2 { x, y };
                if self.map.get(&p).is_some() {
                    if x_first.is_none() {
                        x_first = Some(x);
                    }
                    x_last = Some(x);
                }
            }

            let first = Point2 { x: x_first.unwrap(), y };
            let last = Point2 { x: x_last.unwrap(), y };
            wrap_pairs.insert((first, first + (-1, 0)), (last,  Cardinal::West));
            wrap_pairs.insert((last, last + (1, 0)), (first,  Cardinal::East));
        }

        // Wrap columns
        for x in bounds.x.0 ..= bounds.x.1 {
            let mut y_first = None;
            let mut y_last = None;
            for y in bounds.y.0-1 ..= bounds.y.1+1 {
                let p = Point2 { x, y };
                if self.map.get(&p).is_some() {
                    if y_first.is_none() {
                        y_first = Some(y);
                    }
                    y_last = Some(y);
                }
            }

            let first = Point2 { x, y: y_first.unwrap() };
            let last = Point2 { x, y: y_last.unwrap() };
            wrap_pairs.insert((first, first + (0, -1)), (last,  Cardinal::North));
            wrap_pairs.insert((last, last + (0, 1)), (first,  Cardinal::South));
        }

        wrap_pairs
    }

    /*
        Starting at the start point and moving right, trace out perimeter of the cube net, noting concave and convex corners as we go.
        Start point and moving right are an easy guaranteed place to begin because of the problem definition (start is leftmost of top row).
        Since we're moving clockwise, concave corners are when we turn left, convex corners are when we turn right.
        From each concave corner re-trace the perimeter in forwards and backwards, pairing coordinates along the way. These coordinates are connected in the 3d space.
        Stop re-tracing when either of the traces are at convex corners at the same time. This might seem shaky but I think it's actually solid,
            since if you are tracing one edge to the next you have to stay on one of the faces (turn once) but go to a new face (go straight). I suspect this
            works not just for cubes but any solid object where all faces are at right angles.
    */
    fn wrap_pairs_3d(&self) -> HashMap<(Point2, Point2), (Point2, Cardinal)> {
        let mut perimeter: Vec<(Point2, Cardinal)> = vec![(self.start, Cardinal::East)];
        let mut concave_corners: Vec<Point2> = Vec::new();
        let mut convex_corners: Vec<Point2> = vec![self.start];
        let mut curr_pos = self.start;
        let mut curr_dir = Cardinal::East;

        // Find the perimeter. To make all the logic easier (keep side lengths equal), convex corners are double-counted.
        // Concave corners are also included just to serve as markers so that later we can find their positions as
        // starting points for the wrap mapping, despite that they don't have access to an out-of-bounds point.
        // Record the direction of travel along with the perimeter, which means points that are double counted should
        // have a different direction each time.
        loop {
            // First try turning left
            let next_dir = curr_dir.turn(Turn::Left);
            let next_pos = curr_pos.step(next_dir, 1);
            if self.map.get(&next_pos).is_some() {
                // We can turn left, so do so. This is a concave corner so write that down.
                perimeter.push((next_pos, next_dir));
                concave_corners.push(curr_pos);
                curr_pos = next_pos;
                curr_dir = next_dir;
                continue;
            }

            // Next try continuing straight
            let next_dir = curr_dir;
            let next_pos = curr_pos.step(next_dir, 1);
            if self.map.get(&next_pos).is_some() {
                // We can continue straight, so do so
                perimeter.push((next_pos, next_dir));
                curr_pos = next_pos;
                curr_dir = next_dir;
                continue;
            }

            // Last try turning right
            let next_dir = curr_dir.turn(Turn::Right);
            let next_pos = curr_pos.step(next_dir, 1);
            if self.map.get(&next_pos).is_some() {
                // We can turn right, so do so. This is a convex corner so record the current position with the new direction
                // before recording the new position (current position is the corner and is double counted, we already did
                // the current position and current direction last time). But first, check if the perimeter has ended.
                if (curr_pos, next_dir) == (self.start, Cardinal::East) {
                    // This by definition is where the perimeter loop ends.
                    break;
                }
                perimeter.push((curr_pos, next_dir));
                perimeter.push((next_pos, next_dir));
                convex_corners.push(curr_pos);
                curr_pos = next_pos;
                curr_dir = next_dir;
                continue;
            }

            // Uh oh, no way to continue
            panic!("Couldn't go left, straight, or right from {curr_pos}");
        }

        // (curr point, out-of-bounds next point) -> (wrapped next point, wrapped next dir)
        let mut wrap_pairs: HashMap<(Point2, Point2), (Point2, Cardinal)> = HashMap::new();

        for corner in concave_corners {
            // Go clockwise (a) and counter-clockwise (b)
            let corner_idx = perimeter.iter().position(|(p, _)| *p == corner).unwrap();
            let a_iter = perimeter[corner_idx+1..].iter().chain(perimeter[..corner_idx].iter());
            let b_iter = perimeter[..corner_idx].iter().rev().chain(perimeter[corner_idx..].iter().rev());
            for ((a_pt, a_dir), (b_pt, b_dir)) in iter::zip(a_iter, b_iter) {
                // Store the wrapped pairs
                let a_left = a_pt.step(a_dir.turn(Turn::Left), 1);
                wrap_pairs.insert((*a_pt, a_left), (*b_pt, b_dir.turn(Turn::Right)));
                assert!(!self.map.contains_key(&a_left));

                // Turn::Left on the b-trace looks like a bug but it isn't. Regardless of which way we iterate through
                // the perimeter, the out-of-bounds direction is always what we originally recorded.
                let b_left = b_pt.step(b_dir.turn(Turn::Left), 1);
                wrap_pairs.insert((*b_pt, b_left), (*a_pt, a_dir.turn(Turn::Right)));
                assert!(!self.map.contains_key(&b_left));

                // If both points are convex corners then don't continue further
                if convex_corners.contains(&a_pt) && convex_corners.contains(&b_pt) {
                    break;
                }
            }
        }

        wrap_pairs
    }

    fn trace_path(&self, wrap_pairs: &HashMap<(Point2, Point2), (Point2, Cardinal)>) -> i32 {
        let mut curr_pos = self.start;
        let mut curr_dir = Cardinal::East;

        for step in &self.path {
            match step {
                Step::Forward(n) => {
                    for _ in 0..*n {
                        let mut next_pos = curr_pos.step(curr_dir, 1);
                        let mut next_dir = curr_dir;

                        let tile = self.map.get(&next_pos);
                        if tile.is_none() {
                            // We ran off the edge of the map, wrap around to the opposite side
                            let (new_pos, new_dir) = wrap_pairs[&(curr_pos, next_pos)];
                            next_pos = new_pos;
                            next_dir = new_dir;
                        }

                        match self.map.get(&next_pos) {
                            Some(Tile::Solid) => break,
                            Some(Tile::Open) => {
                                curr_pos = next_pos;
                                curr_dir = next_dir;
                            }
                            _ => panic!("Somehow ran off the map"),
                        }
                    }
                }
                Step::TurnLeft => curr_dir = curr_dir.turn(Turn::Left),
                Step::TurnRight => curr_dir = curr_dir.turn(Turn::Right),
            }
        }

        1000 * (curr_pos.y + 1)
            + 4 * (curr_pos.x + 1)
            + match curr_dir {
                Cardinal::North => 3,
                Cardinal::South => 1,
                Cardinal::East => 0,
                Cardinal::West => 2,
            }
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Board {
    let input = input.replace('\r', "");
    Board::from_str(&input)
}

#[aoc(day22, part1)]
pub fn part1(input: &Board) -> i32 {
    let wrap_pairs = input.wrap_pairs_2d();
    let password = input.trace_path(&wrap_pairs);
    assert_eq!(password, 117102);
    password
}

#[aoc(day22, part2)]
pub fn part2(input: &Board) -> i32 {
    let wrap_pairs = input.wrap_pairs_3d();
    let password = input.trace_path(&wrap_pairs);
    assert_eq!(password, 135297);
    password
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_trace_path_2d() {
        let input = input_generator(EXAMPLE_INPUT);
        let wrap_pairs = input.wrap_pairs_2d();
        let password = input.trace_path(&wrap_pairs);
        assert_eq!(password, 6032);
    }

    #[test]
    fn test_trace_path_3d() {
        let input = input_generator(EXAMPLE_INPUT);
        let wrap_pairs = input.wrap_pairs_3d();
        let password = input.trace_path(&wrap_pairs);
        assert_eq!(password, 5031);
    }
}
