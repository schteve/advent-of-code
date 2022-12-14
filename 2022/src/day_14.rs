/*
    --- Day 14: Regolith Reservoir ---
    The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a little path that leads behind the waterfall.

    Correction: the distress signal leads you behind a giant waterfall! There seems to be a large cave system here, and the signal definitely leads further inside.

    As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand begins pouring into the cave! If you don't quickly figure out where the sand is going, you could quickly become trapped!

    Fortunately, your familiarity with analyzing the path of falling material will come in handy here. You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

    Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point. For example:

    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
    This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines. (Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)

    The sand is pouring into the cave from point 500,0.

    Drawing rock as #, air as ., and the source of the sand as +, this becomes:


    4     5  5
    9     0  0
    4     0  3
    0 ......+...
    1 ..........
    2 ..........
    3 ..........
    4 ....#...##
    5 ....#...#.
    6 ..###...#.
    7 ........#.
    8 ........#.
    9 #########.
    Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.

    A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.

    So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    ......o.#.
    #########.
    The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    .....oo.#.
    #########.
    After a total of five units of sand have come to rest, they form this pattern:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ......o.#.
    ....oooo#.
    #########.
    After a total of 22 units of sand:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ....#ooo#.
    ..###ooo#.
    ....oooo#.
    ...ooooo#.
    #########.
    Finally, only two more units of sand can possibly come to rest:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ...o#ooo#.
    ..###ooo#.
    ....oooo#.
    .o.ooooo#.
    #########.
    Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom, falling into the endless void. Just for fun, the path any new sand takes before falling forever is shown here with ~:

    .......+...
    .......~...
    ......~o...
    .....~ooo..
    ....~#ooo##
    ...~o#ooo#.
    ..~###ooo#.
    ..~..oooo#.
    .~o.ooooo#.
    ~#########.
    ~..........
    ~..........
    ~..........
    Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

    --- Part Two ---
    You realize you misread the scan. There isn't an endless void at the bottom of the scan - there's floor, and you're standing on it!

    You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan.

    In the example above, the highest y coordinate of any point is 9, and so the floor is at y=11. (This is as if your scan contained one extra rock path like -infinity,11 -> infinity,11.) With the added floor, the example above now looks like this:

            ...........+........
            ....................
            ....................
            ....................
            .........#...##.....
            .........#...#......
            .......###...#......
            .............#......
            .............#......
            .....#########......
            ....................
    <-- etc #################### etc -->
    To find somewhere safe to stand, you'll need to simulate falling sand until a unit of sand comes to rest at 500,0, blocking the source entirely and stopping the flow of sand into the cave. In the example above, the situation finally looks like this after 93 units of sand come to rest:

    ............o............
    ...........ooo...........
    ..........ooooo..........
    .........ooooooo.........
    ........oo#ooo##o........
    .......ooo#ooo#ooo.......
    ......oo###ooo#oooo......
    .....oooo.oooo#ooooo.....
    ....oooooooooo#oooooo....
    ...ooo#########ooooooo...
    ..ooooo.......ooooooooo..
    #########################
    Using your scan, simulate the falling sand until the source of the sand becomes blocked. How many units of sand come to rest?
*/

use std::{collections::HashSet, str::FromStr};

use common::{Mode, Point2};

pub struct Paths {
    paths: Vec<Vec<Point2>>,
}

impl Paths {
    fn from_str(input: &str) -> Self {
        let paths = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(Point2::from_str)
                    .collect::<Result<_, _>>()
                    .unwrap()
            })
            .collect();
        Self { paths }
    }
}

struct Cave {
    map: HashSet<Point2>,
    lowest: i32,
}

impl Cave {
    const SAND_SOURCE: Point2 = Point2 { x: 500, y: 0 };

    fn from_paths(paths: &Paths) -> Self {
        let map: HashSet<Point2> = paths
            .paths
            .iter()
            .flat_map(|path| {
                path.windows(2).flat_map(|points| {
                    let a = points[0];
                    let b = points[1];
                    assert!(
                        a.x == b.x || a.y == b.y,
                        "{a} and {b} do not produce orthogonal lines"
                    );

                    let top = a.y.min(b.y);
                    let bottom = a.y.max(b.y);
                    let left = a.x.min(b.x);
                    let right = a.x.max(b.x);
                    (top..=bottom).flat_map(move |y| (left..=right).map(move |x| Point2 { x, y }))
                })
            })
            .collect();

        let lowest = map.iter().map(|p| p.y).max().unwrap();

        Self { map, lowest }
    }

    fn is_air(&self, p: &Point2, mode: Mode) -> bool {
        match mode {
            Mode::M1 => !self.map.contains(p),
            Mode::M2 => !self.map.contains(p) && p.y < self.lowest + 2,
        }
    }

    fn sim_sand(&mut self, mode: Mode) -> usize {
        let mut path_stack: Vec<Point2> = vec![Self::SAND_SOURCE];
        let mut count = 0;

        'outer: while let Some(mut curr) = path_stack.pop() {
            loop {
                let next = [(0, 1), (-1, 1), (1, 1)]
                    .into_iter()
                    .map(|step| curr + step)
                    .find(|next| self.is_air(next, mode));
                if let Some(next) = next {
                    path_stack.push(curr);
                    curr = next;
                } else {
                    let new = self.map.insert(curr);
                    assert!(new, "Sand came to rest at a point without air: {curr}");
                    break;
                }

                if mode == Mode::M1 && curr.y > self.lowest {
                    break 'outer;
                }
            }
            count += 1;
        }

        count
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Paths {
    Paths::from_str(input)
}

#[aoc(day14, part1)]
pub fn part1(input: &Paths) -> usize {
    let mut cave = Cave::from_paths(input);
    let count = cave.sim_sand(Mode::M1);
    assert_eq!(count, 672);
    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Paths) -> usize {
    let mut cave = Cave::from_paths(input);
    let count = cave.sim_sand(Mode::M2);
    assert_eq!(count, 26831);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_sim_sand() {
        let input = input_generator(EXAMPLE_INPUT);

        let mut cave = Cave::from_paths(&input);
        let count = cave.sim_sand(Mode::M1);
        assert_eq!(count, 24);

        let mut cave = Cave::from_paths(&input);
        let count = cave.sim_sand(Mode::M2);
        assert_eq!(count, 93);
    }
}
