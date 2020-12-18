/*
    --- Day 17: Conway Cubes ---
    As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.

    The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

    The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.

    In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.

    The energy source then proceeds to boot up by executing six cycles.

    Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

    During a cycle, all cubes simultaneously change their state according to the following rules:

    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.

    For example, consider the following initial state:

    .#.
    ..#
    ###
    Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)

    Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):

    Before any cycles:

    z=0
    .#.
    ..#
    ###


    After 1 cycle:

    z=-1
    #..
    ..#
    .#.

    z=0
    #.#
    .##
    .#.

    z=1
    #..
    ..#
    .#.


    After 2 cycles:

    z=-2
    .....
    .....
    ..#..
    .....
    .....

    z=-1
    ..#..
    .#..#
    ....#
    .#...
    .....

    z=0
    ##...
    ##...
    #....
    ....#
    .###.

    z=1
    ..#..
    .#..#
    ....#
    .#...
    .....

    z=2
    .....
    .....
    ..#..
    .....
    .....


    After 3 cycles:

    z=-2
    .......
    .......
    ..##...
    ..###..
    .......
    .......
    .......

    z=-1
    ..#....
    ...#...
    #......
    .....##
    .#...#.
    ..#.#..
    ...#...

    z=0
    ...#...
    .......
    #......
    .......
    .....##
    .##.#..
    ...#...

    z=1
    ..#....
    ...#...
    #......
    .....##
    .#...#.
    ..#.#..
    ...#...

    z=2
    .......
    .......
    ..##...
    ..###..
    .......
    .......
    .......
    After the full six-cycle boot process completes, 112 cubes are left in the active state.

    Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?

    --- Part Two ---
    For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.

    The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.

    Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.

    The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.

    For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)

    Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:

    Before any cycles:

    z=0, w=0
    .#.
    ..#
    ###


    After 1 cycle:

    z=-1, w=-1
    #..
    ..#
    .#.

    z=0, w=-1
    #..
    ..#
    .#.

    z=1, w=-1
    #..
    ..#
    .#.

    z=-1, w=0
    #..
    ..#
    .#.

    z=0, w=0
    #.#
    .##
    .#.

    z=1, w=0
    #..
    ..#
    .#.

    z=-1, w=1
    #..
    ..#
    .#.

    z=0, w=1
    #..
    ..#
    .#.

    z=1, w=1
    #..
    ..#
    .#.


    After 2 cycles:

    z=-2, w=-2
    .....
    .....
    ..#..
    .....
    .....

    z=-1, w=-2
    .....
    .....
    .....
    .....
    .....

    z=0, w=-2
    ###..
    ##.##
    #...#
    .#..#
    .###.

    z=1, w=-2
    .....
    .....
    .....
    .....
    .....

    z=2, w=-2
    .....
    .....
    ..#..
    .....
    .....

    z=-2, w=-1
    .....
    .....
    .....
    .....
    .....

    z=-1, w=-1
    .....
    .....
    .....
    .....
    .....

    z=0, w=-1
    .....
    .....
    .....
    .....
    .....

    z=1, w=-1
    .....
    .....
    .....
    .....
    .....

    z=2, w=-1
    .....
    .....
    .....
    .....
    .....

    z=-2, w=0
    ###..
    ##.##
    #...#
    .#..#
    .###.

    z=-1, w=0
    .....
    .....
    .....
    .....
    .....

    z=0, w=0
    .....
    .....
    .....
    .....
    .....

    z=1, w=0
    .....
    .....
    .....
    .....
    .....

    z=2, w=0
    ###..
    ##.##
    #...#
    .#..#
    .###.

    z=-2, w=1
    .....
    .....
    .....
    .....
    .....

    z=-1, w=1
    .....
    .....
    .....
    .....
    .....

    z=0, w=1
    .....
    .....
    .....
    .....
    .....

    z=1, w=1
    .....
    .....
    .....
    .....
    .....

    z=2, w=1
    .....
    .....
    .....
    .....
    .....

    z=-2, w=2
    .....
    .....
    ..#..
    .....
    .....

    z=-1, w=2
    .....
    .....
    .....
    .....
    .....

    z=0, w=2
    ###..
    ##.##
    #...#
    .#..#
    .###.

    z=1, w=2
    .....
    .....
    .....
    .....
    .....

    z=2, w=2
    .....
    .....
    ..#..
    .....
    .....
    After the full six-cycle boot process completes, 848 cubes are left in the active state.

    Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?
*/

use std::collections::HashSet;

type Point2D = (i32, i32);
type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);
type DimRange = (i32, i32);

pub struct PocketDimension2D {
    squares: HashSet<Point2D>,
}

impl PocketDimension2D {
    fn from_string(input: &str) -> Self {
        let mut squares = HashSet::new();
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    squares.insert((x as i32, y as i32));
                }
            }
        }
        Self { squares }
    }
}

struct PocketDimension3D {
    cubes: HashSet<Point3D>,
}

impl PocketDimension3D {
    fn from_2d(pd: &PocketDimension2D) -> Self {
        let cubes = pd
            .squares
            .iter()
            .map(|point| (point.0, point.1, 0))
            .collect();
        Self { cubes }
    }

    fn get_range(&self) -> (DimRange, DimRange, DimRange) {
        let mut cubes_iter = self.cubes.iter();
        if let Some(first) = cubes_iter.next() {
            cubes_iter.fold(
                ((first.0, first.0), (first.1, first.1), (first.2, first.2)),
                |(acc_x, acc_y, acc_z), p| {
                    (
                        (acc_x.0.min(p.0), acc_x.1.max(p.0)),
                        (acc_y.0.min(p.1), acc_y.1.max(p.1)),
                        (acc_z.0.min(p.2), acc_z.1.max(p.2)),
                    )
                },
            )
        } else {
            panic!("No cubes in list");
        }
    }

    fn neighbors(point: &Point3D) -> Vec<Point3D> {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if (x == 0 && y == 0 && z == 0) == false {
                        neighbors.push((point.0 + x, point.1 + y, point.2 + z));
                    }
                }
            }
        }
        assert_eq!(neighbors.len(), 26);
        neighbors
    }

    fn step(&mut self) {
        let mut new_cubes = HashSet::new();

        let (x_range, y_range, z_range) = self.get_range();
        for z in z_range.0 - 1..=z_range.1 + 1 {
            for y in y_range.0 - 1..=y_range.1 + 1 {
                for x in x_range.0 - 1..=x_range.1 + 1 {
                    let cube = (x, y, z);
                    let count = Self::neighbors(&cube)
                        .iter()
                        .filter(|n| self.cubes.contains(n))
                        .count();
                    if self.cubes.contains(&cube) {
                        // Active. 2 or 3 to stay alive, otherwise die.
                        if count == 2 || count == 3 {
                            new_cubes.insert(cube);
                        }
                    } else {
                        // Inactive. 3 to become alive, otherwise stay dead.
                        if count == 3 {
                            new_cubes.insert(cube);
                        }
                    }
                }
            }
        }

        self.cubes = new_cubes;
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn count_active_cubes(&self) -> usize {
        self.cubes.len()
    }
}

impl std::fmt::Display for PocketDimension3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range, z_range) = self.get_range();
        for z in z_range.0..=z_range.1 {
            writeln!(f, "z={}", z)?;
            for y in y_range.0..=y_range.1 {
                for x in x_range.0..=x_range.1 {
                    if self.cubes.contains(&(x, y, z)) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct PocketDimension4D {
    hypercubes: HashSet<Point4D>,
}

impl PocketDimension4D {
    fn from_2d(pd: &PocketDimension2D) -> Self {
        let hypercubes = pd
            .squares
            .iter()
            .map(|point| (point.0, point.1, 0, 0))
            .collect();
        Self { hypercubes }
    }

    fn get_range(&self) -> (DimRange, DimRange, DimRange, DimRange) {
        let mut hypercubes_iter = self.hypercubes.iter();
        if let Some(first) = hypercubes_iter.next() {
            hypercubes_iter.fold(
                (
                    (first.0, first.0),
                    (first.1, first.1),
                    (first.2, first.2),
                    (first.3, first.3),
                ),
                |(acc_x, acc_y, acc_z, acc_w), p| {
                    (
                        (acc_x.0.min(p.0), acc_x.1.max(p.0)),
                        (acc_y.0.min(p.1), acc_y.1.max(p.1)),
                        (acc_z.0.min(p.2), acc_z.1.max(p.2)),
                        (acc_w.0.min(p.3), acc_w.1.max(p.3)),
                    )
                },
            )
        } else {
            panic!("No cubes in list");
        }
    }

    fn neighbors(point: &Point4D) -> Vec<Point4D> {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if (x == 0 && y == 0 && z == 0 && w == 0) == false {
                            neighbors.push((point.0 + x, point.1 + y, point.2 + z, point.3 + w));
                        }
                    }
                }
            }
        }
        assert_eq!(neighbors.len(), 80);
        neighbors
    }

    fn step(&mut self) {
        let mut new_hypercubes = HashSet::new();

        let (x_range, y_range, z_range, w_range) = self.get_range();
        for w in w_range.0 - 1..=w_range.1 + 1 {
            for z in z_range.0 - 1..=z_range.1 + 1 {
                for y in y_range.0 - 1..=y_range.1 + 1 {
                    for x in x_range.0 - 1..=x_range.1 + 1 {
                        let hypercube = (x, y, z, w);
                        let count = Self::neighbors(&hypercube)
                            .iter()
                            .filter(|n| self.hypercubes.contains(n))
                            .count();
                        if self.hypercubes.contains(&hypercube) {
                            // Active. 2 or 3 to stay alive, otherwise die.
                            if count == 2 || count == 3 {
                                new_hypercubes.insert(hypercube);
                            }
                        } else {
                            // Inactive. 3 to become alive, otherwise stay dead.
                            if count == 3 {
                                new_hypercubes.insert(hypercube);
                            }
                        }
                    }
                }
            }
        }

        self.hypercubes = new_hypercubes;
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn count_active_cubes(&self) -> usize {
        self.hypercubes.len()
    }
}

impl std::fmt::Display for PocketDimension4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range, z_range, w_range) = self.get_range();
        for w in w_range.0..=w_range.1 {
            for z in z_range.0..=z_range.1 {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in y_range.0..=y_range.1 {
                    for x in x_range.0..=x_range.1 {
                        if self.hypercubes.contains(&(x, y, z, w)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> PocketDimension2D {
    PocketDimension2D::from_string(input)
}

#[aoc(day17, part1)]
pub fn part1(input: &PocketDimension2D) -> usize {
    let mut pocket_dimension = PocketDimension3D::from_2d(input);
    pocket_dimension.simulate(6);
    let active_cubes = pocket_dimension.count_active_cubes();
    println!("Active cubes after reboot: {}", active_cubes);
    assert_eq!(active_cubes, 401);
    active_cubes
}

#[aoc(day17, part2)]
pub fn part2(input: &PocketDimension2D) -> usize {
    let mut pocket_dimension = PocketDimension4D::from_2d(input);
    pocket_dimension.simulate(6);
    let active_cubes = pocket_dimension.count_active_cubes();
    println!("Active cubes after reboot: {}", active_cubes);
    assert_eq!(active_cubes, 2224);
    active_cubes
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
.#.
..#
###";

    #[test]
    fn test_input_generator() {
        let input = input_generator(EXAMPLE_INPUT);

        let pocket_dimension = PocketDimension3D::from_2d(&input);
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=0
.#.
..#
###"
        );

        let pocket_dimension = PocketDimension4D::from_2d(&input);
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=0, w=0
.#.
..#
###"
        );
    }

    #[test]
    fn test_step_3d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension3D::from_2d(&input);
        pocket_dimension.step();
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#."
        );
    }

    #[test]
    fn test_simulate_3d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension3D::from_2d(&input);
        pocket_dimension.simulate(3);
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
......."
        );
    }

    #[test]
    fn test_count_active_cubes_3d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension3D::from_2d(&input);
        pocket_dimension.simulate(6);
        assert_eq!(pocket_dimension.count_active_cubes(), 112);
    }

    #[test]
    fn test_step_4d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension4D::from_2d(&input);
        pocket_dimension.step();
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=-1, w=-1
#..
..#
.#.

z=0, w=-1
#..
..#
.#.

z=1, w=-1
#..
..#
.#.

z=-1, w=0
#..
..#
.#.

z=0, w=0
#.#
.##
.#.

z=1, w=0
#..
..#
.#.

z=-1, w=1
#..
..#
.#.

z=0, w=1
#..
..#
.#.

z=1, w=1
#..
..#
.#."
        );
    }

    #[test]
    fn test_simulate_4d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension4D::from_2d(&input);
        pocket_dimension.simulate(2);
        assert_eq!(
            pocket_dimension.to_string().trim(),
            "\
z=-2, w=-2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=0, w=-1
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-1, w=0
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
....."
        );
    }

    #[test]
    fn test_count_active_cubes_4d() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut pocket_dimension = PocketDimension4D::from_2d(&input);
        pocket_dimension.simulate(6);
        assert_eq!(pocket_dimension.count_active_cubes(), 848);
    }
}
