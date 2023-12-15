/*
    --- Day 12: Hot Springs ---
    You finally reach the hot springs! You can see steam rising from secluded areas attached to the primary, ornate building.

    As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot springs, weren't you?" You indicate that this definitely looks like hot springs to you.

    "Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."

    You look in the direction the researcher is pointing and suddenly notice the massive metal helixes towering overhead. "This way!"

    It only takes you a few more steps to reach the main gate of the massive fenced-off area containing the springs. You go through the gate and into a small administrative building.

    "Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're having a lava shortage at the moment." You ask about the missing machine parts for Desert Island.

    "Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not until we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"

    "Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal operation, but we should be able to find one springy enough to launch you up there!"

    There's just one problem - many of the springs have fallen into disrepair, so they're not actually sure which springs would even be safe to use! Worse yet, their condition records of which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged records.

    In the giant field just outside, the springs are arranged into rows. For each row, the condition records show every spring and whether it is operational (.) or damaged (#). This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.

    However, the engineer that produced the condition records also duplicated some of this information in a different format! After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row. This list always accounts for every damaged spring, and each number is the entire size of its contiguous group (that is, groups are always separated by at least one operational spring: #### would always be 4, never 2,2).

    So, condition records with no unknown spring conditions might look like this:

    #.#.### 1,1,3
    .#...#....###. 1,1,3
    .#.###.#.###### 1,3,1,6
    ####.#...#... 4,1,1
    #....######..#####. 1,6,5
    .###.##....# 3,2,1
    However, the condition records are partially damaged; some of the springs' conditions are actually unknown (?). For example:

    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    Equipped with this information, it is your job to figure out how many different arrangements of operational and broken springs fit the given criteria in each row.

    In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and three broken springs (in that order) can appear in that row: the first three unknown springs must be broken, then operational, then broken (#.#), making the whole row #.#.###.

    The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different arrangements. The last ? must always be broken (to satisfy the final contiguous group of three broken springs), and each ?? must hide exactly one of the two broken springs. (Neither ?? could be both broken springs or they would form a single contiguous group of two; if that were true, the numbers afterward would have been 2,3 instead.) Since each ?? can either be #. or .#, there are four possible arrangements of springs.

    The last line is actually consistent with ten different arrangements! Because the first number is 3, the first and second ? must both be . (if either were #, the first number would have to be 4 or higher). However, the remaining run of unknown spring conditions have many different ways they could hold groups of two and one broken springs:

    ?###???????? 3,2,1
    .###.##.#...
    .###.##..#..
    .###.##...#.
    .###.##....#
    .###..##.#..
    .###..##..#.
    .###..##...#
    .###...##.#.
    .###...##..#
    .###....##.#
    In this example, the number of possible arrangements for each row is:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 4 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 1 arrangement
    ????.######..#####. 1,6,5 - 4 arrangements
    ?###???????? 3,2,1 - 10 arrangements
    Adding all of the possible arrangement counts together produces a total of 21 arrangements.

    For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. What is the sum of those counts?

    --- Part Two ---
    As you look out at the field of springs, you feel like there are way more springs than the condition records list. When you examine the records, you discover that they were actually folded up this whole time!

    To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?) and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).

    So, this row:

    .# 1
    Would become:

    .#?.#?.#?.#?.# 1,1,1,1,1
    The first line of the above example would become:

    ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
    In the above example, after unfolding, the number of possible arrangements for some rows is now much larger:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 16384 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 16 arrangements
    ????.######..#####. 1,6,5 - 2500 arrangements
    ?###???????? 3,2,1 - 506250 arrangements
    After unfolding, adding all of the possible arrangement counts together produces 525152.

    Unfold your condition records; what is the new sum of possible arrangement counts?
*/

use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum OkOrNo {
    Operational,
    Damaged,
    Unknown,
}

impl OkOrNo {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unknown: {c}"),
        }
    }

    #[allow(dead_code)]
    fn to_char(self) -> char {
        match self {
            OkOrNo::Operational => '.',
            OkOrNo::Damaged => '#',
            OkOrNo::Unknown => '?',
        }
    }
}

#[derive(Clone)]
pub struct ConditionRecord {
    springs: Vec<OkOrNo>,
    groups: Vec<u32>,
}

impl ConditionRecord {
    fn mega(self) -> Self {
        let mut springs = Vec::with_capacity(self.springs.len() * 5 + 4);
        let mut groups = Vec::with_capacity(self.groups.len() * 5);

        springs.extend(&self.springs);
        groups.extend(&self.groups);
        for _ in 0..4 {
            springs.push(OkOrNo::Unknown);
            springs.extend(&self.springs);
            groups.extend(&self.groups);
        }
        Self { springs, groups }
    }
}

impl From<&str> for ConditionRecord {
    fn from(value: &str) -> Self {
        let (left, right) = value.trim().split_once(' ').unwrap();
        let springs = left.chars().map(OkOrNo::from_char).collect();
        let groups = right.split(',').map(|s| s.parse().unwrap()).collect();

        Self { springs, groups }
    }
}

type WaysCache<'a, 'b> = HashMap<(&'a [OkOrNo], &'b [u32], Option<u32>), u64>;

fn ways<'a, 'b>(
    cache: &mut WaysCache<'a, 'b>,
    springs: &'a [OkOrNo],
    groups: &'b [u32],
    count: Option<u32>,
) -> u64 {
    if let Some(cached) = cache.get(&(springs, groups, count)) {
        return *cached;
    }

    let mut retval = 0;
    if springs.is_empty() {
        if groups.is_empty() && (count.is_none() || count == Some(0)) {
            // Successfully placed all groups
            retval = 1;
        } else {
            // We ran out of springs and still had groups to deal with
            retval = 0;
        }
    } else {
        let mut err = false;
        let spring = springs[0];

        if matches!(spring, OkOrNo::Operational | OkOrNo::Unknown) {
            if count.is_some() && count.unwrap() > 0 {
                if matches!(spring, OkOrNo::Operational) {
                    // Interrupted a run of damaged springs
                    err = true;
                }
            } else {
                // Either we aren't tracking a run of damaged springs, or this is the operational separator after one
                retval += ways(cache, &springs[1..], groups, None);
            }
        }

        if matches!(spring, OkOrNo::Damaged | OkOrNo::Unknown) && !err {
            match count {
                None => {
                    if groups.is_empty() {
                        if matches!(spring, OkOrNo::Damaged) {
                            // Too many runs
                            err = true;
                        }
                    } else {
                        // Start a new run
                        retval += ways(cache, &springs[1..], &groups[1..], Some(groups[0] - 1));
                    }
                }
                Some(0) => {
                    if matches!(spring, OkOrNo::Damaged) {
                        // Overran
                        err = true;
                    }
                }
                Some(x) => {
                    // Continue the run
                    retval += ways(cache, &springs[1..], groups, Some(x - 1));
                }
            }
        }

        if err {
            retval = 0;
        }
    }

    cache.insert((springs, groups, count), retval);
    retval
}

fn all_ways(records: Vec<ConditionRecord>) -> u64 {
    records
        .into_iter()
        .map(|cr| {
            let mut cache = HashMap::new();
            ways(&mut cache, &cr.springs, &cr.groups, None)
        })
        .sum()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<ConditionRecord> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[ConditionRecord]) -> u64 {
    let records = input.to_vec();
    let value = all_ways(records);
    assert_eq!(value, 7361);
    value
}

#[aoc(day12, part2)]
pub fn part2(input: &[ConditionRecord]) -> u64 {
    let records = input.iter().cloned().map(|cr| cr.mega()).collect();
    let value = all_ways(records);
    assert_eq!(value, 83317216247365);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_all_ways() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = all_ways(input);
        assert_eq!(value, 21);
    }

    #[test]
    fn test_mega_ways() {
        let input = input_generator(EXAMPLE_INPUT);
        let mega_input: Vec<ConditionRecord> = input.into_iter().map(|cr| cr.mega()).collect();
        let value = all_ways(mega_input);
        assert_eq!(value, 525152);
    }
}
