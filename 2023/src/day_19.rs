/*
    --- Day 19: Aplenty ---
    The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

    As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

    To start, each part is rated in each of four categories:

    x: Extremely cool looking
    m: Musical (it makes a noise when you hit it)
    a: Aerodynamic
    s: Shiny
    Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

    Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

    Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
    Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
    Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
    Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).
    If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

    The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

    {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
    {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
    {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
    {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
    {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A
    Ultimately, three parts are accepted. Adding up the x, m, a, and s rating for each of the accepted parts gives 7540 for the part with x=787, 4623 for the part with x=2036, and 6951 for the part with x=2127. Adding all of the ratings for all of the accepted parts gives the sum total of 19114.

    Sort through all of the parts you've been given; what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?

    --- Part Two ---
    Even with your help, the sorting process still isn't fast enough.

    One of the Elves comes up with a new plan: rather than sort parts individually through all of these workflows, maybe you can figure out in advance which combinations of ratings will be accepted or rejected.

    Each of the four ratings (x, m, a, s) can have an integer value ranging from a minimum of 1 to a maximum of 4000. Of all possible distinct combinations of ratings, your job is to figure out which ones will be accepted.

    In the above example, there are 167409079868000 distinct combinations of ratings that will be accepted.

    Consider only your list of workflows; the list of part ratings that the Elves wanted you to sort is no longer relevant. How many distinct combinations of ratings will be accepted by the Elves' workflows?
*/

use common::{to_owned, trim_start};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, one_of, u32},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone)]
enum Op {
    Lt,
    Gt,
}

impl Op {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, op) = alt((value(Self::Lt, char('<')), value(Self::Gt, char('>'))))(input)?;
        Ok((input, op))
    }
}

struct Condition {
    field: char,
    op: Op,
    val: u32,
}

impl Condition {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (field, op, val)) = tuple((one_of("xmas"), Op::parser, u32))(input)?;

        Ok((input, Self { field, op, val }))
    }
}

struct Rule {
    condition: Option<Condition>,
    next: String,
}

impl Rule {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (condition, next)) = pair(
            opt(terminated(Condition::parser, char(':'))),
            to_owned(alpha1),
        )(input)?;

        Ok((input, Self { condition, next }))
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parser(input: &str) -> IResult<&str, (String, Self)> {
        let (input, (name, rules)) = trim_start(pair(
            terminated(to_owned(alpha1), char('{')),
            terminated(separated_list1(char(','), Rule::parser), char('}')),
        ))(input)?;

        Ok((input, (name, Self { rules })))
    }

    fn apply(&self, part: &Part) -> &str {
        for rule in &self.rules {
            if let Some(cond) = &rule.condition {
                let part_val = match cond.field {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    x => panic!("Invalid cond field: {x}"),
                };
                match cond.op {
                    Op::Lt => {
                        if part_val < cond.val {
                            return &rule.next;
                        }
                    }
                    Op::Gt => {
                        if part_val > cond.val {
                            return &rule.next;
                        }
                    }
                }
            } else {
                return &rule.next;
            }
        }
        unreachable!()
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (x, m, a, s)) = trim_start(delimited(
            char('{'),
            tuple((
                preceded(tag("x="), u32),
                preceded(tag(",m="), u32),
                preceded(tag(",a="), u32),
                preceded(tag(",s="), u32),
            )),
            char('}'),
        ))(input)?;

        Ok((input, Self { x, m, a, s }))
    }

    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

pub struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl System {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (workflow_pairs, parts)) =
            pair(many1(Workflow::parser), many1(Part::parser))(input)?;

        let workflows: HashMap<String, Workflow> = workflow_pairs.into_iter().collect();

        Ok((input, Self { workflows, parts }))
    }

    fn process(&self) -> u32 {
        let mut rating_total = 0;

        for part in &self.parts {
            let mut curr_wf = "in";
            loop {
                curr_wf = self.workflows.get(curr_wf).unwrap().apply(part);
                match curr_wf {
                    "A" => {
                        rating_total += part.rating();
                        break;
                    }
                    "R" => {
                        break;
                    }
                    _ => (),
                }
            }
        }

        rating_total
    }

    fn count_accepted(&self, workflow_name: &str, range: Range4) -> u64 {
        match workflow_name {
            "A" => return range.volume(),
            "R" => return 0,
            _ => (),
        }

        let mut curr_range = range;
        let mut count = 0;

        let workflow = self.workflows.get(workflow_name).unwrap();
        for rule in &workflow.rules {
            if let Some(cond) = &rule.condition {
                let (passed, failed) = curr_range.split(cond);
                count += self.count_accepted(&rule.next, passed);
                curr_range = failed;
            } else {
                count += self.count_accepted(&rule.next, curr_range);
                return count;
            }
        }

        panic!("Workflow rules did not end in unconditional jump");
    }

    fn combinations(&self) -> u64 {
        self.count_accepted(
            "in",
            Range4 {
                x: 1..=4000,
                m: 1..=4000,
                a: 1..=4000,
                s: 1..=4000,
            },
        )
    }
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        Self::parser(value).unwrap().1
    }
}

#[derive(Clone)]
struct Range4 {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl Range4 {
    #[rustfmt::skip]
    fn split(self, cond: &Condition) -> (Self, Self) {
        let field = match cond.field {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            x => panic!("Invalid field: {x}"),
        };

        let (pass, fail) = match cond.op {
            Op::Lt => (*field.start()..=cond.val - 1, cond.val..=*field.end()),
            Op::Gt => (cond.val + 1..=*field.end(), *field.start()..=cond.val),
        };

        match cond.field {
            'x' => (Self { x: pass, ..self.clone() }, Self { x: fail, ..self }),
            'm' => (Self { m: pass, ..self.clone() }, Self { m: fail, ..self }),
            'a' => (Self { a: pass, ..self.clone() }, Self { a: fail, ..self }),
            's' => (Self { s: pass, ..self.clone() }, Self { s: fail, ..self }),
            x => panic!("Invalid field: {x}"),
        }
    }

    #[rustfmt::skip]
    fn volume(self) -> u64 {
        let x = (*self.x.end() - *self.x.start() + 1) as u64;
        let m = (*self.m.end() - *self.m.start() + 1) as u64;
        let a = (*self.a.end() - *self.a.start() + 1) as u64;
        let s = (*self.s.end() - *self.s.start() + 1) as u64;
        x * m * a * s
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> System {
    input.into()
}

#[aoc(day19, part1)]
pub fn part1(input: &System) -> u32 {
    let value = input.process();
    assert_eq!(value, 377025);
    value
}

#[aoc(day19, part2)]
pub fn part2(input: &System) -> u64 {
    let value = input.combinations();
    assert_eq!(value, 135506683246673);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_process() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.process();
        assert_eq!(value, 19114);
    }

    #[test]
    fn test_combinations() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = input.combinations();
        assert_eq!(value, 167409079868000);
    }
}
