/*
    --- Day 6: Chronal Coordinates ---
    The device on your wrist beeps several times, and once again you feel like you're falling.

    "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."

    The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.

    If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.

    Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).

    Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:

    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:

    ..........
    .A........
    ..........
    ........C.
    ...D......
    .....E....
    .B........
    ..........
    ..........
    ........F.
    This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:

    aaaaa.cccc
    aAaaa.cccc
    aaaddecccc
    aadddeccCc
    ..dDdeeccc
    bb.deEeecc
    bBb.eeee..
    bbb.eeefff
    bbb.eeffff
    bbb.ffffFf
    Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.

    In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.

    What is the size of the largest area that isn't infinite?
*/

use common::Point2;
use std::{collections::HashMap, str::FromStr};

struct LandingZone {
    coordinates: Vec<Point2>,
    area: HashMap<Point2, usize>,
}

impl LandingZone {
    fn from_string(input: &str) -> Self {
        let coordinates: Vec<Point2> = input
            .lines()
            .map(|line| Point2::from_str(line).unwrap())
            .collect();
        Self {
            coordinates,
            area: HashMap::new(),
        }
    }

    fn closest_coord(&self, point: Point2) -> usize {
        let closest = self
            .coordinates
            .iter()
            .enumerate()
            .min_by_key(|&(_i, &coord)| Point2::manhattan(point, coord))
            .unwrap();
        closest.0
    }

    fn get_range(&self) -> ((i32, i32), (i32, i32)) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for p in &self.coordinates {
            if p.x < x_range.0 {
                x_range = (p.x, x_range.1);
            } else if p.x > x_range.1 {
                x_range = (x_range.0, p.x);
            }

            if p.y < y_range.0 {
                y_range = (p.y, y_range.1);
            } else if p.y > y_range.1 {
                y_range = (y_range.0, p.y);
            }
        }

        (x_range, y_range)
    }

    fn scan(&mut self) {
        let (x_range, y_range) = self.get_range();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                let p = Point2 { x, y };
                let closest = self.closest_coord(p);
                self.area.insert(p, closest);
            }
        }
    }

    fn get_largest_finite(&mut self) -> u32 {
        // Count size of each area
        let mut original_size = vec![0; self.coordinates.len()];
        self.area
            .values()
            .for_each(|&value| original_size[value] += 1);

        // Add a ring to the outside of the area then count the size of each area again.
        let (x_range, y_range) = self.get_range();
        let x_range = (x_range.0 - 1, x_range.1 + 1);
        let y_range = (y_range.0 - 1, y_range.1 + 1);
        // Top and bottom
        for x in x_range.0..=x_range.1 {
            let p = Point2 { x, y: y_range.0 };
            let closest = self.closest_coord(p);
            self.area.insert(p, closest);

            let p = Point2 { x, y: y_range.1 };
            let closest = self.closest_coord(p);
            self.area.insert(p, closest);
        }
        // Left and right
        for y in y_range.0..=y_range.1 {
            let p = Point2 { x: x_range.0, y };
            let closest = self.closest_coord(p);
            self.area.insert(p, closest);

            let p = Point2 { x: x_range.1, y };
            let closest = self.closest_coord(p);
            self.area.insert(p, closest);
        }

        let mut new_size = vec![0; self.coordinates.len()];
        self.area.values().for_each(|&value| new_size[value] += 1);

        // Anything that increased in size is infinite and is disqualified.
        let largest = original_size
            .iter()
            .zip(new_size.iter())
            .filter_map(|(&orig, &new)| if orig == new { Some(orig) } else { None })
            .max()
            .unwrap();
        largest
    }
}

#[aoc(day6, part1)]
pub fn solve(input: &str) -> u32 {
    let mut landing_zone = LandingZone::from_string(input);
    landing_zone.scan();
    let largest = landing_zone.get_largest_finite();
    println!("Largest finite area: {}", largest);
    assert_eq!(largest, 3882);
    largest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_finite() {
        let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let mut landing_zone = LandingZone::from_string(input);
        landing_zone.scan();
        let largest = landing_zone.get_largest_finite();
        assert_eq!(largest, 17);
    }
}
