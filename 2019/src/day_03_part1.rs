/*
    --- Day 3: Crossed Wires ---
    The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.

    Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).

    The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.

    For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:

    ...........
    ...........
    ...........
    ....+----+.
    ....|....|.
    ....|....|.
    ....|....|.
    .........|.
    .o-------+.
    ...........
    Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:

    ...........
    .+-----+...
    .|.....|...
    .|..+--X-+.
    .|..|..|.|.
    .|.-X--+.|.
    .|..|....|.
    .|.......|.
    .o-------+.
    ...........
    These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.

    Here are a few more examples:

    R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
    R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
    What is the Manhattan distance from the central port to the closest intersection?
*/

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
struct Coord {
    p: Point,
}

fn get_coords_from_path(path: Vec<&str>) -> Vec<Coord> {
    let mut coords = Vec::new();
    let mut current_coord = Coord { p: Point { x: 0, y: 0 } };

    for &segment in path.iter() {
        let direction = segment.as_bytes()[0];
        let count = segment[1..].parse::<u32>().unwrap();

        // println!("Segment {}, {}", direction, count);

        for _ in 0..count {
            match direction as char {
                'R' => {
                    current_coord.p.x += 1;
                    coords.push(current_coord);
                }
                'L' => {
                    current_coord.p.x -= 1;
                    coords.push(current_coord);
                }
                'U' => {
                    current_coord.p.y += 1;
                    coords.push(current_coord);
                }
                'D' => {
                    current_coord.p.y -= 1;
                    coords.push(current_coord);
                }
                _   => {
                    println!("FAIL");
                    break;
                }
            }
        }
    }

    coords
}

fn intersection(coords1: Vec<Coord>, coords2: Vec<Coord>) -> Vec<Coord> {
    let mut intersection = Vec::new();

    for &c1 in coords1.iter() {
        if coords2.contains(&c1) {
            println!("Intersect = p ({}, {})", c1.p.x, c1.p.y);
            intersection.push(c1);
        }
    }

    // println!("Intersection = {:?}", intersection);
    intersection
}

fn manhattan_distance(p: Point) -> u32 {
    let distance = p.x.abs() + p.y.abs();
    distance as u32
}

fn best_intersection(path1: Vec<&str>, path2: Vec<&str>) -> u32 {
    let path1_coords = get_coords_from_path(path1);
    let path2_coords = get_coords_from_path(path2);

    let intersect_coords = intersection(path1_coords, path2_coords);

    let closest_distance = intersect_coords.iter()
        .map(|&c| manhattan_distance(c.p))
        .min()
        .unwrap();
    closest_distance
}

#[aoc(day3, part1)]
pub fn solve(input: &str) -> u32 {
    let paths: Vec<&str> = input.lines().collect();
    let path_a = paths[0].split(",").collect::<Vec<&str>>();
    let path_b = paths[1].split(",").collect::<Vec<&str>>();

    let distance = best_intersection(path_a, path_b);
    println!("Distance = {}", distance);
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best_intersection() {
        let path1_a = "R8,U5,L5,D3".split(",").collect::<Vec<&str>>();
        let path1_b = "U7,R6,D4,L4".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path1_a, path1_b), 6);

        let path2_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect::<Vec<&str>>();
        let path2_b = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path2_a, path2_b), 159);

        let path3_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(",").collect::<Vec<&str>>();
        let path3_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path3_a, path3_b), 135);
    }
}
