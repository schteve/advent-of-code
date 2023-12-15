/*

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
