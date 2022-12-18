/*
    --- Day 18: Boiling Boulders ---
    You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.

    Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.

    Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).

    Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z position.

    To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube. So, if your scan were only two adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single side covered and five sides exposed, a total surface area of 10 sides.

    Here's a larger example:

    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5
    In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is 64.

    What is the surface area of your scanned lava droplet?

    --- Part Two ---
    Something seems off about your calculation. The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.

    Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond. The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.

    In the larger example above, exactly one cube of air is trapped within the lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is 58.

    What is the exterior surface area of your scanned lava droplet?
*/

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use common::Point3;

fn calc_surface_area(pixels: &[Point3]) -> usize {
    // This map tracks the interstitials - just the exposed faces.
    // Double the pixel coords so that odd numbers are faces and even numbers are pixel centers.
    let mut set: HashSet<Point3> = HashSet::new();

    for pixel in pixels {
        let double_pixel = Point3::from((pixel.x * 2, pixel.y * 2, pixel.z * 2));
        for offset in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            // Insert the face, if it was already there then go back and remove it.
            // Since each face can only be shared by two pixels, there's no danger of
            // re-adding it after it was removed.
            let face = double_pixel + offset;
            let new_val = set.insert(face);
            if !new_val {
                set.remove(&face);
            }
        }
    }

    set.len()
}

#[derive(Debug, PartialEq)]
enum Kind {
    Lava,
    Air,
    Water,
}

fn calc_flooded(pixels: &[Point3]) -> usize {
    let mut map: HashMap<Point3, Kind> = pixels.iter().map(|p| (*p, Kind::Lava)).collect();
    let range = Point3::get_range(pixels).unwrap();

    // Pre-fill with air. Note we leave a border to account for areas the outside may be closed off.
    for x in range.x.0 - 1..=range.x.1 + 1 {
        for y in range.y.0 - 1..=range.y.1 + 1 {
            for z in range.z.0 - 1..=range.z.1 + 1 {
                let p = Point3::from((x, y, z));
                map.entry(p).or_insert(Kind::Air);
            }
        }
    }

    // Everything is either air or lava at this point. Flood fill with water from the outside.
    // Follow the air, and since each lava-water face is uniquely "owned" by a water pixel we can just
    // count as we go.
    let mut count = 0;
    let start = (range.x.0, range.y.0, range.z.0).into();
    let mut frontier: Vec<Point3> = vec![start];
    while let Some(curr) = frontier.pop() {
        for offset in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let adj = curr + offset;
            match map.get(&adj) {
                Some(Kind::Air) => {
                    map.insert(adj, Kind::Water);
                    frontier.push(adj);
                }
                Some(Kind::Lava) => count += 1,
                _ => (),
            }
        }
    }

    count
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Point3> {
    input
        .lines()
        .map(Point3::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Point3]) -> usize {
    let area = calc_surface_area(input);
    assert_eq!(area, 3390);
    area
}

#[aoc(day18, part2)]
pub fn part2(input: &[Point3]) -> usize {
    let area = calc_flooded(input);
    assert_eq!(area, 2058);
    area
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_calc_surface_area() {
        let input = input_generator(EXAMPLE_INPUT);
        let area = calc_surface_area(&input);
        assert_eq!(area, 64);
    }

    #[test]
    fn test_calc_flooded() {
        let input = input_generator(EXAMPLE_INPUT);
        let area = calc_flooded(&input);
        assert_eq!(area, 58);
    }
}
