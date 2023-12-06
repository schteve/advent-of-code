/*
    --- Day 5: If You Give A Seed A Fertilizer ---
    You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

    "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

    "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

    "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

    You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

    The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

    For example:

    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

    The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

    Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

    Consider again the example seed-to-soil map:

    50 98 2
    52 50 48
    The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

    The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

    Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

    So, the entire list of seed numbers and their corresponding soil numbers looks like this:

    seed  soil
    0     0
    1     1
    ...   ...
    48    48
    49    49
    50    52
    51    53
    ...   ...
    96    98
    97    99
    98    50
    99    51
    With this map, you can look up the soil number required for each initial seed number:

    Seed number 79 corresponds to soil number 81.
    Seed number 14 corresponds to soil number 14.
    Seed number 55 corresponds to soil number 57.
    Seed number 13 corresponds to soil number 13.
    The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

    Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
    So, the lowest location number in this example is 35.

    What is the lowest location number that corresponds to any of the initial seed numbers?

    --- Part Two ---
    Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

    The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

    seeds: 79 14 55 13
    This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

    Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

    In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

    Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?
*/

use std::ops::Range;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult,
};

use common::{to_owned, trim, unsigned};

struct MapRange {
    dst: u64,
    src: u64,
    len: u64,
}

impl MapRange {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (dst, src, len)) =
            tuple((trim(unsigned), trim(unsigned), trim(unsigned)))(input)?;

        Ok((input, Self { src, dst, len }))
    }

    fn convert(&self, other: u64) -> Option<u64> {
        if other >= self.src {
            let offset = other - self.src;
            if offset < self.len {
                return Some(self.dst + offset);
            }
        }
        None
    }

    fn intersect(&self, range: Range<u64>) -> (Option<Range<u64>>, [Option<Range<u64>>; 2]) {
        if (range.start < (self.src + self.len)) && (range.end > self.src) {
            let start_offset = std::cmp::max(range.start, self.src) - self.src;
            let end_offset = std::cmp::min(range.end, self.src + self.len) - self.src;
            let start = self.dst + start_offset;
            let end = self.dst + end_offset;
            let intersection = Some(start..end);

            let before = if range.start < self.src {
                Some(range.start..self.src)
            } else {
                None
            };

            let after = if range.end > (self.src + self.len) {
                Some(self.src + self.len..range.end)
            } else {
                None
            };

            (intersection, [before, after])
        } else {
            (None, [Some(range), None])
        }
    }
}

struct Map {
    _from: String,
    _to: String,
    map_ranges: Vec<MapRange>,
}

impl Map {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (from, _, to, _, map_ranges)) = trim(tuple((
            to_owned(alpha1),
            tag("-to-"),
            to_owned(alpha1),
            tag(" map:"),
            many1(MapRange::parser),
        )))(input)?;

        Ok((
            input,
            Self {
                _from: from,
                _to: to,
                map_ranges,
            },
        ))
    }

    fn convert(&self, other: u64) -> u64 {
        for range in &self.map_ranges {
            if let Some(mapped) = range.convert(other) {
                return mapped;
            }
        }
        other
    }

    fn convert_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut done_ranges = Vec::new();
        for range in ranges {
            let mut curr_ranges = vec![range];

            for map_range in &self.map_ranges {
                let mut tmp_ranges = Vec::new();

                for curr in curr_ranges.drain(..) {
                    let (done, not_done) = map_range.intersect(curr);
                    done_ranges.extend(done.into_iter());
                    tmp_ranges.extend(not_done.into_iter().flatten());
                }

                curr_ranges.append(&mut tmp_ranges);
            }

            done_ranges.extend(curr_ranges);
        }
        done_ranges
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (seeds, maps)) = pair(
            preceded(tag("seeds: "), separated_list1(space1, unsigned)),
            many1(Map::parser),
        )(input)?;

        Ok((input, Self { seeds, maps }))
    }

    fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.convert(acc)))
            .min()
            .unwrap()
    }

    fn lowest_location_hard(&self) -> u64 {
        let mut ranges: Vec<Range<u64>> = self
            .seeds
            .chunks_exact(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect();

        for map in &self.maps {
            ranges = map.convert_ranges(ranges);
        }

        // All conversions have been done, the lowest value in each range is always the start of the range
        ranges.into_iter().map(|range| range.start).min().unwrap()
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    Almanac::parser(input).unwrap().1
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> u64 {
    let value = input.lowest_location();
    assert_eq!(value, 175622908);
    value
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanac) -> u64 {
    let value = input.lowest_location_hard();
    assert_eq!(value, 5200543);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_lowest_location() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.lowest_location();
        assert_eq!(value, 35);
    }

    #[test]
    fn test_lowest_location_hard() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.lowest_location_hard();
        assert_eq!(value, 46);
    }

    #[test]
    fn test_maprange_intersect() {
        let m0 = MapRange {
            dst: 20,
            src: 10,
            len: 5,
        };
        assert_eq!(m0.intersect(0..5), (None, [Some(0..5), None]));
        assert_eq!(m0.intersect(10..15), (Some(20..25), [None, None]));
        assert_eq!(m0.intersect(5..15), (Some(20..25), [Some(5..10), None]));
        assert_eq!(m0.intersect(10..20), (Some(20..25), [None, Some(15..20)]));
        assert_eq!(
            m0.intersect(0..20),
            (Some(20..25), [Some(0..10), Some(15..20)])
        );
    }
}
