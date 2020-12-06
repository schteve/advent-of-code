/*
    --- Day 6: Custom Customs ---
    As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

    The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

    However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

    abcx
    abcy
    abcz
    In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

    Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b
    This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.
    In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

    For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

    --- Part Two ---
    As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

    You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

    Using the same example as above:

    abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b
    This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
    In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

    For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
*/

use nom::{
    character::complete::{alpha1, line_ending, multispace0},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Group {
    people: Vec<String>,
}

impl Group {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, people) = preceded(
            multispace0,
            separated_list1(line_ending, map(alpha1, |x: &str| x.to_owned())),
        )(input)?;

        Ok((input, Group { people }))
    }

    fn answers1(&self) -> String {
        let mut answers = String::new();
        for c in b'a'..=b'z' {
            if self.people.iter().any(|person| person.contains(c as char)) == true {
                answers.push(c as char);
            }
        }
        answers
    }

    fn answers2(&self) -> String {
        let mut answers = String::new();
        for c in b'a'..=b'z' {
            if self.people.iter().all(|person| person.contains(c as char)) == true {
                answers.push(c as char);
            }
        }
        answers
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
    many1(Group::parser)(input).unwrap().1
}

#[aoc(day6, part1)]
pub fn part1(input: &[Group]) -> usize {
    let count_sum = input.iter().map(|g| g.answers1().len()).sum();
    println!("Sum of any answers: {}", count_sum);
    assert_eq!(count_sum, 6683);
    count_sum
}

#[aoc(day6, part2)]
pub fn part2(input: &[Group]) -> usize {
    let count_sum = input.iter().map(|g| g.answers2().len()).sum();
    println!("Sum of all answers: {}", count_sum);
    assert_eq!(count_sum, 3122);
    count_sum
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_input_generator() {
        let groups = input_generator(EXAMPLE_INPUT);
        assert_eq!(
            groups,
            [
                Group {
                    people: vec!["abc".into()]
                },
                Group {
                    people: vec!["a".into(), "b".into(), "c".into()]
                },
                Group {
                    people: vec!["ab".into(), "ac".into()]
                },
                Group {
                    people: vec!["a".into(), "a".into(), "a".into(), "a".into()]
                },
                Group {
                    people: vec!["b".into()],
                },
            ]
        );
    }

    #[test]
    fn test_group_answers1() {
        let groups = input_generator(EXAMPLE_INPUT);
        let answers: Vec<String> = groups.iter().map(Group::answers1).collect();
        assert_eq!(answers, ["abc", "abc", "abc", "a", "b"]);
        let answers_count: Vec<usize> = answers.iter().map(|s| s.len()).collect();
        assert_eq!(answers_count, [3, 3, 3, 1, 1]);
    }

    #[test]
    fn test_group_answers2() {
        let groups = input_generator(EXAMPLE_INPUT);
        let answers: Vec<String> = groups.iter().map(Group::answers2).collect();
        assert_eq!(answers, ["abc", "", "a", "a", "b"]);
        let answers_count: Vec<usize> = answers.iter().map(|s| s.len()).collect();
        assert_eq!(answers_count, [3, 0, 1, 1, 1]);
    }
}
