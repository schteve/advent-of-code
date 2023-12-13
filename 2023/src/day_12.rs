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
    // Some(true): completely valid
    // Some(false): valid at least up to the first unknown
    // None: not valid
    fn validate(&self) -> Option<bool> {
        enum State {
            Idle,
            Tracking,
            Break,
            AllMatched,
        }
        let mut grp_idx = 0;
        let mut curr_count = 0;
        let mut state = State::Idle;
        for spring in &self.springs {
            match (&state, &spring) {
                (State::Idle, OkOrNo::Operational) => (),
                (State::Idle, OkOrNo::Damaged) => {
                    curr_count = self.groups[grp_idx] - 1;
                    if curr_count == 0 {
                        grp_idx += 1;
                        if grp_idx >= self.groups.len() {
                            state = State::AllMatched;
                        } else {
                            state = State::Break;
                        }
                    } else {
                        state = State::Tracking;
                    }
                }
                (State::Idle, OkOrNo::Unknown) => return Some(false),
                (State::Tracking, OkOrNo::Operational) => return None,
                (State::Tracking, OkOrNo::Damaged) => {
                    curr_count -= 1;
                    if curr_count == 0 {
                        grp_idx += 1;
                        if grp_idx >= self.groups.len() {
                            state = State::AllMatched;
                        } else {
                            state = State::Break;
                        }
                    }
                }
                (State::Tracking, OkOrNo::Unknown) => return Some(false),
                (State::Break, OkOrNo::Operational) => {
                    state = State::Idle;
                }
                (State::Break, OkOrNo::Damaged) => return None,
                (State::Break, OkOrNo::Unknown) => return Some(false),
                (State::AllMatched, OkOrNo::Operational) => (),
                (State::AllMatched, OkOrNo::Damaged) => return None,
                (State::AllMatched, OkOrNo::Unknown) => return Some(false),
            }
        }

        if matches!(state, State::AllMatched) {
            Some(true)
        } else {
            None
        }
    }

    fn ways(&mut self) -> u64 {
        let valid = self.validate();
        match valid {
            Some(true) => 1,
            Some(false) => {
                let first_unk_idx = self
                    .springs
                    .iter()
                    .position(|spring| matches!(spring, OkOrNo::Unknown))
                    .unwrap();

                let mut child_ways = 0;
                self.springs[first_unk_idx] = OkOrNo::Operational;
                if self.validate().is_some() {
                    child_ways += self.ways();
                }

                self.springs[first_unk_idx] = OkOrNo::Damaged;
                if self.validate().is_some() {
                    child_ways += self.ways();
                }
                self.springs[first_unk_idx] = OkOrNo::Unknown;

                child_ways
            }
            None => 0,
        }
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
    assert_eq!(value, 123);
    value
}

/*#[aoc(day12, part2)]
pub fn part2(input: &[ConditionRecord]) -> u64 {
    let value = x(input);
    assert_eq!(value, 456);
    value
}*/

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
}
