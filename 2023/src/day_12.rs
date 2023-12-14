/*

*/

#[derive(Clone, Copy)]
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

fn ways(springs: &[OkOrNo], groups: &[u32]) -> u64 {
    let mut count = None;
    for (i, spring) in springs.iter().enumerate() {
        match spring {
            OkOrNo::Damaged | OkOrNo::Unknown => {
                if count.is_none() {
                    if groups.is_empty() {
                        return 0;
                    } else {
                        count = Some(groups[0]);
                    }
                }

                if count == Some(0) {
                    return 0;
                } else {
                    count = count.map(|x| x - 1);
                }
            }
            OkOrNo::Operational => match count {
                Some(0) => return ways(&springs[i + 1..], &groups[1..]),
                Some(x) => return 0,
                None => (),
            },
        }
    }

    // We handle the cases where unknowns are filled in as damaged
    // how do we handle when they're filled in as operational?
    // maybe save first unknown location and do that after, or do it immediately?
    // somewhere have to add the two cases together.
    // is what i'm doing here fundamentally any different than what i did before?

    if groups.is_empty() {
        1
    } else {
        0
    }
}

fn all_ways(records: Vec<ConditionRecord>) -> u64 {
    records.into_iter().map(|mut cr| cr.ways()).sum()
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
    assert_eq!(value, 456);
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
