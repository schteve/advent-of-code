/*
    --- Day 11: Seating System ---
    Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

    By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

    The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL

    Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.
    Floor (.) never changes; seats don't move, and nobody sits on the floor.

    After one round of these rules, every seat in the example layout becomes occupied:

    #.##.##.##
    #######.##
    #.#.#..#..
    ####.##.##
    #.##.##.##
    #.#####.##
    ..#.#.....
    ##########
    #.######.#
    #.#####.##

    After a second round, the seats with four or more occupied adjacent seats become empty again:

    #.LL.L#.##
    #LLLLLL.L#
    L.L.L..L..
    #LLL.LL.L#
    #.LL.LL.LL
    #.LLLL#.##
    ..L.L.....
    #LLLLLLLL#
    #.LLLLLL.L
    #.#LLLL.##

    This process continues for three more rounds:

    #.##.L#.##
    #L###LL.L#
    L.#.#..#..
    #L##.##.L#
    #.##.LL.LL
    #.###L#.##
    ..#.#.....
    #L######L#
    #.LL###L.L
    #.#L###.##

    #.#L.L#.##
    #LLL#LL.L#
    L.L.L..#..
    #LLL.##.L#
    #.LL.LL.LL
    #.LL#L#.##
    ..L.L.....
    #L#LLLL#L#
    #.LLLLLL.L
    #.#L#L#.##

    #.#L.L#.##
    #LLL#LL.L#
    L.#.L..#..
    #L##.##.L#
    #.#L.LL.LL
    #.#L#L#.##
    ..L.L.....
    #L#L##L#L#
    #.LLLLLL.L
    #.#L#L#.##

    At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

    Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

    --- Part Two ---
    As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

    Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

    .......#.
    ...#.....
    .#.......
    .........
    ..#L....#
    ....#....
    .........
    #........
    ...#.....
    The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

    .............
    .L.L.#.#.#.#.
    .............
    The empty seat below would see no occupied seats:

    .##.##.
    #.#.#.#
    ##...##
    ...L...
    ##...##
    #.#.#.#
    .##.##.
    Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

    Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL

    #.##.##.##
    #######.##
    #.#.#..#..
    ####.##.##
    #.##.##.##
    #.#####.##
    ..#.#.....
    ##########
    #.######.#
    #.#####.##

    #.LL.LL.L#
    #LLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLL#
    #.LLLLLL.L
    #.LLLLL.L#

    #.L#.##.L#
    #L#####.LL
    L.#.#..#..
    ##L#.##.##
    #.##.#L.##
    #.#####.#L
    ..#.#.....
    LLL####LL#
    #.L#####.L
    #.L####.L#

    #.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##LL.LL.L#
    L.LL.LL.L#
    #.LLLLL.LL
    ..L.L.....
    LLLLLLLLL#
    #.LLLLL#.L
    #.L#LL#.L#

    #.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##L#.#L.L#
    L.L#.#L.L#
    #.L####.LL
    ..#.#.....
    LLL###LLL#
    #.LLLLL#.L
    #.L#LL#.L#

    #.L#.L#.L#
    #LLLLLL.LL
    L.L.L..#..
    ##L#.#L.L#
    L.L#.LL.L#
    #.LLLL#.LL
    ..#.L.....
    LLL###LLL#
    #.LLLLL#.L
    #.L#LL#.L#

    Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

    Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
*/

use crate::common::{Mode, Point};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Unknown character: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Floor => '.',
            Self::Empty => 'L',
            Self::Occupied => '#',
        }
    }
}

fn get_tile_flat(p: &Point, x_size: usize, y_size: usize, tiles: &[Tile]) -> Tile {
    if p.x >= 0 && p.x < x_size as i32 && p.y >= 0 && p.y < y_size as i32 {
        let idx = point_to_idx(p, x_size);
        tiles[idx]
    } else {
        Tile::Floor
    }
}

fn point_to_idx(p: &Point, x_size: usize) -> usize {
    p.y as usize * x_size + p.x as usize
}

fn idx_to_point(idx: usize, x_size: usize) -> Point {
    let x = (idx % x_size) as i32;
    let y = (idx / x_size) as i32;
    Point { x, y }
}

#[derive(Clone)]
pub struct WaitingArea {
    tiles: Vec<Tile>,
    x_size: usize,
    y_size: usize,
    active: Vec<Point>,
    neighbors_direct: Vec<Vec<Point>>,
    neighbors_visible: Vec<Vec<Point>>,
}

impl WaitingArea {
    fn from_string(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut x_size: Option<usize> = None;
        for line in input.trim().lines() {
            tiles.extend(line.chars().map(Tile::from_char));

            if let Some(x) = x_size {
                assert_eq!(x, line.len());
            } else {
                x_size = Some(line.len());
            }
        }
        let x_size = x_size.unwrap();
        let y_size = tiles.len() / x_size;

        // Build list of which tiles are active
        let active = tiles
            .iter()
            .enumerate()
            .filter(|(_i, seat)| seat == &&Tile::Empty)
            .map(|(i, _seat)| {
                let x = (i % x_size) as i32;
                let y = (i / x_size) as i32;
                Point { x, y }
            })
            .collect();

        // Build index of non-floor neighbors directly adjacent to each tile
        let neighbors_direct = (0..tiles.len())
            .map(|i| {
                idx_to_point(i, x_size)
                    .adjacents()
                    .filter(|adj| get_tile_flat(adj, x_size, y_size, &tiles) == Tile::Empty)
                    .collect()
            })
            .collect();

        // Build index of non-floor neighbors visible from each tile
        let mut neighbors_visible = Vec::new();
        for i in 0..tiles.len() {
            let mut visible_dirs: Vec<Point> = Vec::new();
            for dir in Point::origin().adjacents() {
                let mut walk = idx_to_point(i, x_size);
                while 0 <= walk.x && walk.x < x_size as i32 && 0 <= walk.y && walk.y < y_size as i32
                {
                    walk += dir;
                    if get_tile_flat(&walk, x_size, y_size, &tiles) == Tile::Empty {
                        visible_dirs.push(walk);
                        break;
                    }
                }
                // If we left the waiting area bounds, nothing is visible so don't save anything
            }
            neighbors_visible.push(visible_dirs);
        }

        Self {
            tiles,
            x_size,
            y_size,
            active,
            neighbors_direct,
            neighbors_visible,
        }
    }

    fn get_tile(&self, p: &Point) -> Tile {
        get_tile_flat(p, self.x_size, self.y_size, &self.tiles)
    }

    fn set_tile(&mut self, p: &Point, tile: Tile) {
        let idx = point_to_idx(p, self.x_size);
        self.tiles[idx] = tile;
    }

    fn count_neighbors_direct(&self, p: &Point) -> usize {
        let idx = point_to_idx(p, self.x_size);
        self.neighbors_direct[idx]
            .iter()
            .filter(|adj| self.get_tile(&adj) == Tile::Occupied)
            .count()
    }

    fn count_neighbors_visible(&self, p: &Point) -> usize {
        let idx = point_to_idx(p, self.x_size);
        self.neighbors_visible[idx]
            .iter()
            .filter(|adj| self.get_tile(&adj) == Tile::Occupied)
            .count()
    }

    fn step(&mut self, mode: Mode) -> bool {
        let mut changes: Vec<(Point, Tile)> = Vec::new();
        for p in self.active.iter() {
            match (mode, self.get_tile(&p)) {
                (_, Tile::Floor) => (),
                (Mode::M1, Tile::Empty) => {
                    // Become occupied if there are no direct neighbors occupied
                    if self.count_neighbors_direct(&p) == 0 {
                        changes.push((*p, Tile::Occupied));
                    }
                }
                (Mode::M1, Tile::Occupied) => {
                    // Become empty if four or more direct neighbors are also occupied
                    if self.count_neighbors_direct(&p) >= 4 {
                        changes.push((*p, Tile::Empty));
                    }
                }
                (Mode::M2, Tile::Empty) => {
                    // Become occupied if there are no visible neighbors occupied
                    if self.count_neighbors_visible(&p) == 0 {
                        changes.push((*p, Tile::Occupied));
                    }
                }
                (Mode::M2, Tile::Occupied) => {
                    // Become empty if five or more visible neighbors are also occupied
                    if self.count_neighbors_visible(&p) >= 5 {
                        changes.push((*p, Tile::Empty));
                    }
                }
            }
        }

        for change in &changes {
            self.set_tile(&change.0, change.1);
        }
        changes.is_empty()
    }

    fn simulate(&mut self, mode: Mode) {
        while self.step(mode) == false {}
    }

    fn total_occupied(&self) -> usize {
        self.tiles
            .iter()
            .filter(|seat| seat == &&Tile::Occupied)
            .count()
    }
}

impl std::fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.y_size as i32 {
            for x in 0..self.x_size as i32 {
                let p = Point { x, y };
                write!(f, "{}", self.get_tile(&p).to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> WaitingArea {
    WaitingArea::from_string(input)
}

#[aoc(day11, part1)]
pub fn part1(input: &WaitingArea) -> usize {
    let mut waiting_area = input.clone();
    waiting_area.simulate(Mode::M1);
    let occupied = waiting_area.total_occupied();
    assert_eq!(occupied, 2183);
    occupied
}

#[aoc(day11, part2)]
pub fn part2(input: &WaitingArea) -> usize {
    let mut waiting_area = input.clone();
    waiting_area.simulate(Mode::M2);
    let occupied = waiting_area.total_occupied();
    assert_eq!(occupied, 1990);
    occupied
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_simulate() {
        let mut waiting_area = input_generator(EXAMPLE_INPUT);
        waiting_area.simulate(Mode::M1);
        let occupied = waiting_area.total_occupied();
        assert_eq!(occupied, 37);
        let expected = input_generator(
            "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        );
        assert_eq!(waiting_area.to_string(), expected.to_string());

        let mut waiting_area = input_generator(EXAMPLE_INPUT);
        waiting_area.simulate(Mode::M2);
        let occupied = waiting_area.total_occupied();
        assert_eq!(occupied, 26);

        let expected = input_generator(
            "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        );
        assert_eq!(waiting_area.to_string(), expected.to_string());
    }
}
