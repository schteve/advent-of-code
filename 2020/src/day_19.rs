/*
    --- Day 19: Monster Messages ---
    You land in an airport surrounded by dense forest. As you walk to your high-speed train, the Elves at the Mythical Information Bureau contact you again. They think their satellite has collected an image of a sea monster! Unfortunately, the connection to the satellite is having problems, and many of the messages sent back from the satellite have been corrupted.

    They sent you a list of the rules valid messages should obey and a list of received messages they've collected so far (your puzzle input).

    The rules for valid messages (the top part of your puzzle input) are numbered and build upon each other. For example:

    0: 1 2
    1: "a"
    2: 1 3 | 3 1
    3: "b"
    Some rules, like 3: "b", simply match a single character (in this case, b).

    The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means that to match rule 0, the text being checked must match rule 1, and the text after the part that matched rule 1 must then match rule 2.

    Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at least one list of sub-rules must match. (The ones that match might be different each time the rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.

    Fortunately, there are no loops in the rules, so the list of possible matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0 matches aab or aba.

    Here's a more interesting example:

    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"
    Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the same (aa or bb), and rule 3 matches two letters that are different (ab or ba).

    Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of letters, one pair with matching letters and one pair with different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.

    Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.

    The received messages (the bottom part of your puzzle input) need to be checked against the rules so you can determine which are valid and which are corrupted. Including the rules and the messages together, this might look like:

    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"

    ababbb
    bababa
    abbbab
    aaabbb
    aaaabbb
    Your goal is to determine the number of messages that completely match rule 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer 2. The whole message must match all of rule 0; there can't be extra unmatched characters in the message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched b on the end.)

    How many messages completely match rule 0?

    --- Part Two ---
    As you look over the list of messages, you realize your matching rules aren't quite right. To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:

    8: 42 | 42 8
    11: 42 31 | 42 11 31
    This small change has a big impact: now, the rules do contain loops, and the list of messages they could hypothetically match is infinite. You'll need to determine how these changes affect which messages are valid.

    Fortunately, many of the rules are unaffected by this change; it might help to start by looking at which rules always match the same set of values and how those rules (especially rules 42 and 31) are used by the new versions of rules 8 and 11.

    (Remember, you only need to handle the rules you have; building a solution that could handle any hypothetical combination of rules would be significantly more difficult.)

    For example:

    42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: "a"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: "b"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1

    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaaaabbaaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    babaaabbbaaabaababbaabababaaab
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba, ababaaaaaabaaab, and ababaaaaabbbaba.

    However, after updating rules 8 and 11, a total of 12 messages match:

    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    After updating rules 8 and 11, how many messages completely match rule 0?
*/

use crate::common::{to_owned, trim_start, unsigned};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char},
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Rule {
    Rules(Vec<Vec<u32>>),
    Value(char),
}

impl Rule {
    fn parser(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                separated_list1(tag(" | "), separated_list1(char(' '), unsigned)),
                Self::Rules,
            ),
            map(delimited(char('"'), anychar, char('"')), Self::Value),
        ))(input)
    }
}

#[derive(Clone, Debug)]
enum Frame<'a> {
    RulesIter(std::slice::Iter<'a, u32>),
    Value(char),
}

#[derive(Clone, Debug)]
struct FrameStack<'a> {
    stack: Vec<Frame<'a>>,
}

impl<'a> FrameStack<'a> {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn from_rule(start_rule: &'a Rule) -> Vec<Self> {
        let mut first = Self::new();
        let mut others = first.process_rule(start_rule);
        others.push(first);
        others
    }

    fn process_rule(&mut self, r: &'a Rule) -> Vec<Self> {
        let mut frame_stacks = Vec::new();
        match r {
            Rule::Rules(rules) => {
                // This rule has multiple rules within it, duplicate the current frame stack and add a new frame to each of them
                let mut rules_iter = rules.iter();
                let first = rules_iter.next(); // Process the first item last so that we can clone self as the base but still re-use its memory
                for rule in rules_iter {
                    let frame = Frame::RulesIter(rule.iter());
                    let mut frame_stack = self.clone();
                    frame_stack.stack.push(frame);
                    frame_stacks.push(frame_stack);
                }
                if let Some(rule) = first {
                    let frame = Frame::RulesIter(rule.iter());
                    self.stack.push(frame);
                }
            }
            Rule::Value(c) => {
                // Add the value to the stack
                let frame = Frame::Value(*c);
                self.stack.push(frame);
            }
        }
        frame_stacks
    }

    fn advance_to_value(&mut self, rules: &'a HashMap<u32, Rule>) -> (Option<bool>, Vec<Self>) {
        let mut done = None; // Some(true) means keep the stack, Some(false) means discard it, None means more to process
        let mut new_frame_stacks = Vec::new();
        match self.stack.last_mut() {
            Some(Frame::RulesIter(ref mut rules_iter)) => {
                if let Some(rule_id) = rules_iter.next() {
                    // This frame has more rule IDs to process
                    new_frame_stacks = self.process_rule(&rules[rule_id]);
                } else {
                    // This frame is done being processed. Remove it from the stack, but keep the stack on the frontier so its next frame can be processed.
                    self.stack.pop();
                }
            }
            Some(Frame::Value(_)) => {
                // This frame is a value, so this stack is done being processed
                done = Some(true);
            }
            None => {
                // This stack matched earlier, but more input exists so it's no longer relevant
                done = Some(false);
            }
        }
        (done, new_frame_stacks)
    }

    fn advance_value(&mut self) {
        self.stack.pop();
    }

    fn matches(&self, msg_char: char) -> bool {
        if let Some(Frame::Value(c)) = self.stack.last() {
            *c == msg_char
        } else {
            panic!("Expected value");
        }
    }

    fn remove_matched_frames(&mut self) {
        while let Some(Frame::RulesIter(ref mut rules_iter)) = self.stack.last_mut() {
            if rules_iter.next().is_none() {
                // This frame was done already, remove it
                self.stack.pop();
            } else {
                // This frame wasn't done yet, nothing left to do on this frame stack
                break;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.stack.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Comms {
    rules: HashMap<u32, Rule>,
    messages: Vec<String>,
}

impl Comms {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (rules, messages)) = pair(
            map(
                many1(trim_start(separated_pair(
                    unsigned,
                    tag(": "),
                    Rule::parser,
                ))),
                |rules_list| rules_list.into_iter().collect(),
            ),
            many0(trim_start(to_owned(alpha1))),
        )(input)?;

        Ok((input, Self { rules, messages }))
    }

    fn match_message(&self, message: &str) -> bool {
        // Initialize the frame stacks
        let mut frame_stacks: Vec<FrameStack> = FrameStack::from_rule(&self.rules[&0]);

        for msg_char in message.chars() {
            // First, process each frame stack until its current frame is at a value.
            let mut frontier_frame_stacks: Vec<FrameStack> = std::mem::take(&mut frame_stacks);
            while let Some(frame_stack) = frontier_frame_stacks.last_mut() {
                let (done, mut new_frame_stacks) = frame_stack.advance_to_value(&self.rules);
                match done {
                    Some(true) => {
                        // Done processing, keep the frame stack
                        let frame_stack = frontier_frame_stacks.pop().unwrap();
                        frame_stacks.push(frame_stack);
                    }
                    Some(false) => {
                        // Done processing, discard the frame stack
                        frontier_frame_stacks.pop();
                    }
                    None => {}
                }
                frontier_frame_stacks.append(&mut new_frame_stacks);
            }

            // Keep only the frame stacks for which the current frame matches the input
            frame_stacks.retain(|frame_stack| frame_stack.matches(msg_char) == true);

            // All remaining stacks match the input, so far. Advance past the value that was just matched on each stack.
            for frame_stack in frame_stacks.iter_mut() {
                frame_stack.advance_value();
            }
        }

        // Post process the stacks. Remove frames that already matched (empty rule iterators).
        for frame_stack in frame_stacks.iter_mut() {
            frame_stack.remove_matched_frames();
        }

        // The match is successful only if all of the input was matched; only stacks that are empty were completely matched with the last character.
        frame_stacks
            .iter()
            .any(|frame_stack| frame_stack.is_complete())
    }

    fn patch_rules(&mut self) {
        self.rules.insert(8, Rule::parser("42 | 42 8").unwrap().1);
        self.rules
            .insert(11, Rule::parser("42 31 | 42 11 31").unwrap().1);
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Comms {
    Comms::parser(input).unwrap().1
}

#[aoc(day19, part1)]
pub fn part1(input: &Comms) -> usize {
    let count = input
        .messages
        .iter()
        .filter(|&m| input.match_message(m))
        .count();
    assert_eq!(count, 272);
    count
}

#[aoc(day19, part2)]
pub fn part2(input: &Comms) -> usize {
    let mut comms = input.clone();
    comms.patch_rules();
    let count = comms
        .messages
        .iter()
        .filter(|&m| comms.match_message(m))
        .count();
    assert_eq!(count, 374);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"";

    static EXAMPLE_INPUT2: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"";

    static EXAMPLE_INPUT3: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    static EXAMPLE_INPUT4: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_match_message() {
        let comms = input_generator(EXAMPLE_INPUT1);
        let messages = [
            ("aab", true),
            ("aba", true),
            ("a", false),
            ("aaba", false),
            ("aaa", false),
            ("bbb", false),
        ];
        for &(m, expected) in messages.iter() {
            assert_eq!(comms.match_message(m), expected);
        }

        let comms = input_generator(EXAMPLE_INPUT2);
        let messages = [
            ("aaaabb", true),
            ("aaabab", true),
            ("abbabb", true),
            ("abbbab", true),
            ("aabaab", true),
            ("aabbbb", true),
            ("abaaab", true),
            ("ababbb", true),
            ("a", false),
            ("aaaabba", false),
            ("baaaabb", false),
            ("bbbbb", false),
        ];
        for &(m, expected) in messages.iter() {
            assert_eq!(comms.match_message(m), expected);
        }
    }

    #[test]
    fn test_count_matches() {
        let comms = input_generator(EXAMPLE_INPUT4);
        let count = comms
            .messages
            .iter()
            .filter(|&m| comms.match_message(m))
            .count();
        assert_eq!(count, 3);

        let mut comms = input_generator(EXAMPLE_INPUT4);
        comms.patch_rules();
        let count = comms
            .messages
            .iter()
            .filter(|&m| comms.match_message(m))
            .count();
        assert_eq!(count, 12);
    }
}
