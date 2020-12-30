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

use crate::common::Point;
use std::collections::HashMap;

#[derive(Clone)]
pub struct WaitingArea {
    seats: HashMap<Point, bool>,
    visible: HashMap<Point, Vec<Point>>,
}

impl WaitingArea {
    fn from_string(input: &str) -> Self {
        let mut seats = HashMap::new();
        let mut p = Point::new();
        for line in input.trim().lines() {
            p.x = 0;
            for c in line.chars() {
                if c == 'L' {
                    seats.insert(p, false);
                } else if c == '#' {
                    seats.insert(p, true);
                }
                p.x += 1;
            }
            p.y += 1;
        }

        // Map which seats are visible in each of the adjacent directions
        let directions = Point::new().adjacents();
        let (x_range, y_range) = Point::get_range(seats.keys()).unwrap();
        let mut visible = HashMap::new();
        for start in seats.keys() {
            let mut visible_dirs: Vec<Point> = Vec::new();
            for dir in &directions {
                let mut walk = *start;
                while x_range.0 <= walk.x
                    && walk.x <= x_range.1
                    && y_range.0 <= walk.y
                    && walk.y <= y_range.1
                {
                    walk += dir;
                    if seats.get(&walk).is_some() {
                        visible_dirs.push(walk);
                        break;
                    }
                }
                // If we left the waiting area bounds, nothing is visible so don't save anything
            }
            visible.insert(*start, visible_dirs);
        }

        Self { seats, visible }
    }

    fn count_occupied(&self, seats: &[Point]) -> usize {
        seats
            .iter()
            .filter(|adj| self.seats.get(&adj) == Some(&true))
            .count()
    }

    fn step1(&mut self) {
        let mut changes: Vec<(Point, bool)> = Vec::new();
        for (&k, &v) in self.seats.iter() {
            if v == true {
                // Is occupied. Becomes empty if four or more adjacent are also occupied.
                if self.count_occupied(&k.adjacents()) >= 4 {
                    changes.push((k, false));
                }
            } else {
                // Empty. Becomes occupied if there are no adjacent occupied.
                if self.count_occupied(&k.adjacents()) == 0 {
                    changes.push((k, true));
                }
            }
        }
        self.seats.extend(changes);
    }

    fn simulate1(&mut self) {
        let mut last_state = String::new();
        while self.to_string() != last_state {
            last_state = self.to_string();
            self.step1();
        }
    }

    fn step2(&mut self) {
        let mut changes: Vec<(Point, bool)> = Vec::new();
        for (&k, &v) in self.seats.iter() {
            if v == true {
                // Is occupied. Becomes empty if five or more visible are also occupied.
                if let Some(vis) = self.visible.get(&k) {
                    if self.count_occupied(&vis) >= 5 {
                        changes.push((k, false));
                    }
                }
            } else {
                // Empty. Becomes occupied if there are no adjacent occupied.
                if let Some(vis) = self.visible.get(&k) {
                    if self.count_occupied(&vis) == 0 {
                        changes.push((k, true));
                    }
                }
            }
        }
        self.seats.extend(changes);
    }

    fn simulate2(&mut self) {
        let mut last_state = String::new();
        while self.to_string() != last_state {
            last_state = self.to_string();
            self.step2();
        }
    }

    fn total_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|&&is_occupied| is_occupied == true)
            .count()
    }
}

impl std::fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = Point::get_range(self.seats.keys()).unwrap();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if let Some(&is_occupied) = self.seats.get(&Point { x, y }) {
                    if is_occupied == true {
                        write!(f, "#")?;
                    } else {
                        write!(f, "L")?;
                    }
                } else {
                    write!(f, ".")?;
                }
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
    waiting_area.simulate1();
    let occupied = waiting_area.total_occupied();
    assert_eq!(occupied, 2183);
    occupied
}

#[aoc(day11, part2)]
pub fn part2(input: &WaitingArea) -> usize {
    let mut waiting_area = input.clone();
    waiting_area.simulate2();
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
    fn test_simulate1() {
        let mut waiting_area = input_generator(EXAMPLE_INPUT);
        waiting_area.simulate1();
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
    }

    #[test]
    fn test_simulate2() {
        let mut waiting_area = input_generator(EXAMPLE_INPUT);
        waiting_area.simulate2();
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
