/*
    --- Day 13: Distress Signal ---
    You climb the hill and again try contacting the Elves. However, you instead receive a signal you weren't expecting: a distress signal.

    Your handheld device must still not be working properly; the packets from the distress signal got decoded out of order. You'll need to re-order the list of received packets (your puzzle input) to decode the message.

    Your list consists of pairs of packets; pairs are separated by a blank line. You need to identify how many pairs of packets are in the right order.

    For example:

    [1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    Packet data consists of lists and integers. Each list starts with [, ends with ], and contains zero or more comma-separated values (either integers or other lists). Each packet is always a list and appears on its own line.

    When comparing two values, the first value is called left and the second value is called right. Then:

    If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
    If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
    If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
    Using these rules, you can determine which of the pairs in the example are in the right order:

    == Pair 1 ==
    - Compare [1,1,3,1,1] vs [1,1,5,1,1]
    - Compare 1 vs 1
    - Compare 1 vs 1
    - Compare 3 vs 5
        - Left side is smaller, so inputs are in the right order

    == Pair 2 ==
    - Compare [[1],[2,3,4]] vs [[1],4]
    - Compare [1] vs [1]
        - Compare 1 vs 1
    - Compare [2,3,4] vs 4
        - Mixed types; convert right to [4] and retry comparison
        - Compare [2,3,4] vs [4]
        - Compare 2 vs 4
            - Left side is smaller, so inputs are in the right order

    == Pair 3 ==
    - Compare [9] vs [[8,7,6]]
    - Compare 9 vs [8,7,6]
        - Mixed types; convert left to [9] and retry comparison
        - Compare [9] vs [8,7,6]
        - Compare 9 vs 8
            - Right side is smaller, so inputs are not in the right order

    == Pair 4 ==
    - Compare [[4,4],4,4] vs [[4,4],4,4,4]
    - Compare [4,4] vs [4,4]
        - Compare 4 vs 4
        - Compare 4 vs 4
    - Compare 4 vs 4
    - Compare 4 vs 4
    - Left side ran out of items, so inputs are in the right order

    == Pair 5 ==
    - Compare [7,7,7,7] vs [7,7,7]
    - Compare 7 vs 7
    - Compare 7 vs 7
    - Compare 7 vs 7
    - Right side ran out of items, so inputs are not in the right order

    == Pair 6 ==
    - Compare [] vs [3]
    - Left side ran out of items, so inputs are in the right order

    == Pair 7 ==
    - Compare [[[]]] vs [[]]
    - Compare [[]] vs []
        - Right side ran out of items, so inputs are not in the right order

    == Pair 8 ==
    - Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
    - Compare 1 vs 1
    - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
        - Compare 2 vs 2
        - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
        - Compare 3 vs 3
        - Compare [4,[5,6,7]] vs [4,[5,6,0]]
            - Compare 4 vs 4
            - Compare [5,6,7] vs [5,6,0]
            - Compare 5 vs 5
            - Compare 6 vs 6
            - Compare 7 vs 0
                - Right side is smaller, so inputs are not in the right order
    What are the indices of the pairs that are already in the right order? (The first pair has index 1, the second pair has index 2, and so on.) In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.

    Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?

    --- Part Two ---
    Now, you just need to put all of the packets in the right order. Disregard the blank lines in your list of received packets.

    The distress signal protocol also requires that you include two additional divider packets:

    [[2]]
    [[6]]
    Using the same rules as before, organize all packets - the ones in your list of received packets as well as the two divider packets - into the correct order.

    For the example above, the result of putting the packets in the correct order is:

    []
    [[]]
    [[[]]]
    [1,1,3,1,1]
    [1,1,5,1,1]
    [[1],[2,3,4]]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [[1],4]
    [[2]]
    [3]
    [[4,4],4,4]
    [[4,4],4,4,4]
    [[6]]
    [7,7,7]
    [7,7,7,7]
    [[8,7,6]]
    [9]
    Afterward, locate the divider packets. To find the decoder key for this distress signal, you need to determine the indices of the two divider packets and multiply them together. (The first packet is at index 1, the second packet is at index 2, and so on.) In this example, the divider packets are 10th and 14th, and so the decoder key is 140.

    Organize all of the packets into the correct order. What is the decoder key for the distress signal?
*/

use std::{cmp::Ordering, fmt, iter};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    multi::{many1, separated_list0},
    sequence::delimited,
    IResult,
};

#[derive(Clone)]
pub enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Packet {
    // OK fine, I'll use a parser
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, packet) = common::trim_start(alt((
            map(common::unsigned::<u8>, Self::Int),
            map(
                delimited(
                    char('['),
                    separated_list0(char(','), Self::parser),
                    char(']'),
                ),
                Self::List,
            ),
        )))(input)?;

        Ok((input, packet))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Int(a), Self::List(_)) => Self::List(vec![Self::Int(*a)]).cmp(other),
            (Self::List(_), Self::Int(b)) => self.cmp(&Self::List(vec![Self::Int(*b)])),
            (Self::List(a), Self::List(b)) => {
                for (a, b) in iter::zip(a.iter(), b.iter()) {
                    let ord = a.cmp(b);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}")?,
            Self::List(list) => {
                write!(f, "[")?;
                #[allow(unstable_name_collisions)]
                write!(
                    f,
                    "{}",
                    list.iter()
                        .map(|p| p.to_string())
                        .intersperse(String::from(", "))
                        .collect::<String>()
                )?;
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

fn score_pairs_in_order(packets: &[Packet]) -> u64 {
    packets
        .chunks(2)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            if pair[0] <= pair[1] {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .sum()
}

fn score_sorted_packets(packets: &[Packet]) -> u64 {
    let markers = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];
    packets
        .iter()
        .chain(markers.iter())
        .sorted()
        .enumerate()
        .filter_map(|(idx, p)| {
            if markers.contains(p) {
                Some(idx as u64 + 1)
            } else {
                None
            }
        })
        .product()
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Packet> {
    many1(Packet::parser)(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn part1(input: &[Packet]) -> u64 {
    let score = score_pairs_in_order(input);
    assert_eq!(score, 6484);
    score
}

#[aoc(day13, part2)]
pub fn part2(input: &[Packet]) -> u64 {
    let score = score_sorted_packets(input);
    assert_eq!(score, 19305);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_score_pairs_in_order() {
        let input = input_generator(EXAMPLE_INPUT);
        let score = score_pairs_in_order(&input);
        assert_eq!(score, 13);
    }

    #[test]
    fn test_score_sorted_packets() {
        let input = input_generator(EXAMPLE_INPUT);
        let score = score_sorted_packets(&input);
        assert_eq!(score, 140);
    }
}
