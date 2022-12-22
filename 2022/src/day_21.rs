/*
    --- Day 21: Monkey Math ---
    The monkeys are back! You're worried they're going to try to steal your stuff again, but it seems like they're just holding their ground and making various monkey noises at you.

    Eventually, one of the elephants realizes you don't speak monkey and comes over to interpret. As it turns out, they overheard you talking about trying to find the grove; they can show you a shortcut if you answer their riddle.

    Each monkey is given a job: either to yell a specific number or to yell the result of a math operation. All of the number-yelling monkeys know their number from the start; however, the math operation monkeys need to wait for two other monkeys to yell a number, and those two other monkeys might also be waiting on other monkeys.

    Your job is to work out the number the monkey named root will yell before the monkeys figure it out themselves.

    For example:

    root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32
    Each line contains the name of a monkey, a colon, and then the job of that monkey:

    A lone number means the monkey's job is simply to yell that number.
    A job like aaaa + bbbb means the monkey waits for monkeys aaaa and bbbb to yell each of their numbers; the monkey then yells the sum of those two numbers.
    aaaa - bbbb means the monkey yells aaaa's number minus bbbb's number.
    Job aaaa * bbbb will yell aaaa's number multiplied by bbbb's number.
    Job aaaa / bbbb will yell aaaa's number divided by bbbb's number.
    So, in the above example, monkey drzm has to wait for monkeys hmdt and zczc to yell their numbers. Fortunately, both hmdt and zczc have jobs that involve simply yelling a single number, so they do this immediately: 32 and 2. Monkey drzm can then yell its number by finding 32 minus 2: 30.

    Then, monkey sjmn has one of its numbers (30, from monkey drzm), and already has its other number, 5, from dbpl. This allows it to yell its own number by finding 30 multiplied by 5: 150.

    This process continues until root yells a number: 152.

    However, your actual situation involves considerably more monkeys. What number will the monkey named root yell?

    --- Part Two ---
    Due to some kind of monkey-elephant-human mistranslation, you seem to have misunderstood a few key details about the riddle.

    First, you got the wrong job for the monkey named root; specifically, you got the wrong math operation. The correct operation for monkey root should be =, which means that it still listens for two numbers (from the same two monkeys as before), but now checks that the two numbers match.

    Second, you got the wrong monkey for the job starting with humn:. It isn't a monkey - it's you. Actually, you got the job wrong, too: you need to figure out what number you need to yell so that root's equality check passes. (The number that appears after humn: in your input is now irrelevant.)

    In the above example, the number you need to yell to pass root's equality test is 301. (This causes root to get the same number, 150, from both of its monkeys.)

    What number do you yell to pass root's equality test?
*/

use std::collections::HashMap;

use common::Mode;

#[derive(Clone)]
pub enum Yell {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Yell {
    fn from_str(s: &str) -> Self {
        if let Ok(num) = s.parse() {
            Self::Num(num)
        } else {
            let mut iter = s.split_ascii_whitespace();
            let a = iter.next().unwrap().to_owned();
            let op = iter.next().unwrap();
            let b = iter.next().unwrap().to_owned();
            match op {
                "+" => Self::Add(a, b),
                "-" => Self::Sub(a, b),
                "*" => Self::Mul(a, b),
                "/" => Self::Div(a, b),
                x => panic!("Invalid op: \"{x}\""),
            }
        }
    }

    // When 'a' is the unsolved portion e.g. x - 3 = val
    fn inverse_a(&self, val: i64, b: i64) -> i64 {
        match self {
            Yell::Num(_) => panic!("Can't inverse a number"),
            Yell::Add(_, _) => val - b,
            Yell::Sub(_, _) => val + b,
            Yell::Mul(_, _) => val / b,
            Yell::Div(_, _) => val * b,
        }
    }

    // When 'b' is the unsolved portion e.g. 2 + x = val
    fn inverse_b(&self, val: i64, a: i64) -> i64 {
        match self {
            Yell::Num(_) => panic!("Can't inverse a number"),
            Yell::Add(_, _) => val - a,
            Yell::Sub(_, _) => a - val,
            Yell::Mul(_, _) => val / a,
            Yell::Div(_, _) => a / val,
        }
    }
}

struct MonkeyGang {
    gang: HashMap<String, Yell>,
    mode: Mode,
}

impl MonkeyGang {
    fn from_names_yells(names_yells: &[(String, Yell)], mode: Mode) -> Self {
        let gang = names_yells.iter().cloned().collect();
        Self { gang, mode }
    }

    fn evaluate_monkey(&self, name: &str) -> Option<i64> {
        if self.mode == Mode::M2 && name == "humn" {
            return None;
        }

        let yell = self.gang.get(name).expect("Monkey doesn't exist");
        match yell {
            Yell::Num(n) => Some(*n),
            Yell::Add(a, b) => self
                .evaluate_monkey(a)
                .zip(self.evaluate_monkey(b))
                .map(|(aa, bb)| aa + bb),
            Yell::Sub(a, b) => self
                .evaluate_monkey(a)
                .zip(self.evaluate_monkey(b))
                .map(|(aa, bb)| aa - bb),
            Yell::Mul(a, b) => self
                .evaluate_monkey(a)
                .zip(self.evaluate_monkey(b))
                .map(|(aa, bb)| aa * bb),
            Yell::Div(a, b) => self
                .evaluate_monkey(a)
                .zip(self.evaluate_monkey(b))
                .map(|(aa, bb)| aa / bb),
        }
    }

    fn humn_after_all(&self) -> i64 {
        let yell = self.gang.get("root").unwrap();
        let (a, b) = match yell {
            Yell::Num(_) => panic!("How to test equality on a single number??"),
            Yell::Add(a, b) | Yell::Sub(a, b) | Yell::Mul(a, b) | Yell::Div(a, b) => (a, b),
        };

        match (self.evaluate_monkey(a), self.evaluate_monkey(b)) {
            (Some(_), Some(_)) => panic!("Both sides are already solved"),
            (Some(aa), None) => self.find_expected_value(b, aa),
            (None, Some(bb)) => self.find_expected_value(a, bb),
            (None, None) => panic!("Neither side is solvable"),
        }
    }

    fn find_expected_value(&self, name: &str, expected_value: i64) -> i64 {
        if name == "humn" {
            return expected_value;
        }

        // First figure out which side is the one with the humn
        let yell = self.gang.get(name).unwrap();
        let (a, b) = match yell {
            Yell::Num(_) => panic!("Found a leaf number when I was expecting something unsolved"),
            Yell::Add(a, b) | Yell::Sub(a, b) | Yell::Mul(a, b) | Yell::Div(a, b) => (a, b),
        };

        match (self.evaluate_monkey(a), self.evaluate_monkey(b)) {
            (Some(_), Some(_)) => panic!("Both sides are already solved"),
            (Some(aa), None) => {
                let next_expected_value = yell.inverse_b(expected_value, aa);
                self.find_expected_value(b, next_expected_value)
            }
            (None, Some(bb)) => {
                let next_expected_value = yell.inverse_a(expected_value, bb);
                self.find_expected_value(a, next_expected_value)
            }
            (None, None) => panic!("Neither side is solvable"),
        }
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(String, Yell)> {
    input
        .lines()
        .map(|line| {
            let (name, yell_str) = line.split_once(": ").unwrap();
            (name.to_owned(), Yell::from_str(yell_str))
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &[(String, Yell)]) -> i64 {
    let gang = MonkeyGang::from_names_yells(input, Mode::M1);
    let root = gang.evaluate_monkey("root").unwrap();
    assert_eq!(root, 85616733059734);
    root
}

#[aoc(day21, part2)]
pub fn part2(input: &[(String, Yell)]) -> i64 {
    let gang = MonkeyGang::from_names_yells(input, Mode::M2);
    let humn = gang.humn_after_all();
    assert_eq!(humn, 3560324848168);
    humn
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_evaluate_monkey() {
        let input = input_generator(EXAMPLE_INPUT);
        let gang = MonkeyGang::from_names_yells(&input, Mode::M1);
        let root = gang.evaluate_monkey("root").unwrap();
        assert_eq!(root, 152);
    }

    #[test]
    fn test_inverse_a() {
        let s = String::new();
        assert_eq!(Yell::Add(s.clone(), s.clone()).inverse_a(8, 5), 3); // x + 5 = 8
        assert_eq!(Yell::Sub(s.clone(), s.clone()).inverse_a(8, 5), 13); // x - 5 = 8
        assert_eq!(Yell::Mul(s.clone(), s.clone()).inverse_a(8, 2), 4); // x * 2 = 8
        assert_eq!(Yell::Div(s.clone(), s.clone()).inverse_a(8, 2), 16); // x / 2 = 8

        assert_eq!(Yell::Add(s.clone(), s.clone()).inverse_b(8, 5), 3); // 5 + x = 8
        assert_eq!(Yell::Sub(s.clone(), s.clone()).inverse_b(8, 5), -3); // 5 - x = 8
        assert_eq!(Yell::Mul(s.clone(), s.clone()).inverse_b(8, 2), 4); // 2 * x = 8
        assert_eq!(Yell::Div(s.clone(), s.clone()).inverse_b(8, 24), 3); // 24 / x = 8
    }

    #[test]
    fn test_humn_after_all() {
        let input = input_generator(EXAMPLE_INPUT);
        let gang = MonkeyGang::from_names_yells(&input, Mode::M2);
        let humn = gang.humn_after_all();
        assert_eq!(humn, 301);
    }
}
