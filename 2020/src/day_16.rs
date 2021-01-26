/*
    --- Day 16: Ticket Translation ---
    As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.

    Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.

    You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).

    The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).

    Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:

    .--------------------------------------------------------.
    | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
    |                                                        |
    | ??: 301  ??: 302             ???????: 303      ??????? |
    | ??: 401  ??: 402           ???? ????: 403    ????????? |
    '--------------------------------------------------------'
    Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!

    Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.

    For example, suppose you have the following notes:

    class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12
    It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.

    Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?

    --- Part Two ---
    Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.

    Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.

    For example, suppose you have the following notes:

    class: 0-1 or 4-19
    row: 0-5 or 8-19
    seat: 0-13 or 16-19

    your ticket:
    11,12,13

    nearby tickets:
    3,9,18
    15,1,5
    5,14,9
    Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.

    Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?
*/

use crate::common::{to_owned, trim, trim_start, unsigned};
use nom::{
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, line_ending},
        is_alphabetic, is_space,
    },
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn range_parser(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(separated_pair(unsigned, char('-'), unsigned), |(a, b)| {
        a..=b
    })(input)
}

#[derive(Debug)]
pub struct Notes {
    rules: HashMap<String, Vec<RangeInclusive<u32>>>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Notes {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (rules_list, my_ticket, nearby_tickets)) = tuple((
            many1(pair(
                terminated(
                    trim_start(to_owned(take_while1(|c: char| {
                        is_alphabetic(c as u8) || is_space(c as u8)
                    }))),
                    tag(": "),
                ),
                separated_list1(tag(" or "), range_parser),
            )),
            preceded(
                trim(tag("your ticket:")),
                separated_list1(char(','), unsigned),
            ),
            preceded(
                trim(tag("nearby tickets:")),
                separated_list1(line_ending, separated_list1(char(','), unsigned)),
            ),
        ))(input)?;

        let mut rules = HashMap::new();
        rules.extend(rules_list);

        Ok((
            input,
            Self {
                rules,
                my_ticket,
                nearby_tickets,
            },
        ))
    }

    fn field_in_ranges(field: &u32, ranges: &[RangeInclusive<u32>]) -> bool {
        ranges.iter().any(|r| r.contains(field))
    }

    fn field_is_valid(&self, field: &u32) -> bool {
        self.rules
            .values()
            .any(|ranges| Notes::field_in_ranges(field, ranges))
    }

    fn ticket_is_valid(&self, ticket: &[u32]) -> bool {
        ticket.iter().all(|field| self.field_is_valid(field))
    }

    fn ticket_scanning_error_rate(&self) -> u32 {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|field| self.field_is_valid(field) == false)
            })
            .sum()
    }

    fn find_field_ordering(&self) -> Vec<String> {
        let valid_tickets: Vec<Vec<u32>> = self
            .nearby_tickets
            .iter()
            .filter(|&ticket| self.ticket_is_valid(ticket) == true)
            .cloned()
            .collect();

        // Find all possible fields for each rule
        let mut could_be: HashMap<String, Vec<bool>> = HashMap::new();
        for (name, ranges) in self.rules.iter() {
            let possibilities: Vec<bool> = (0..valid_tickets[0].len())
                .map(|field_idx| {
                    valid_tickets
                        .iter()
                        .all(|ticket| Notes::field_in_ranges(&ticket[field_idx], ranges))
                })
                .collect();
            could_be.insert(name.clone(), possibilities);
        }

        // This completely assumes that each rule has either 1, 2, 3, 4... possibilities, so
        // it's only a matter of finding the next entry that has only one possibility left.
        let mut ordering: Vec<Option<String>> = vec![None; valid_tickets[0].len()];
        while ordering.iter().any(|x| x.is_none()) == true {
            for (name, possibilities) in &could_be {
                let mut only_one_idx = None;
                for (i, p) in possibilities.iter().enumerate() {
                    if *p == true && ordering[i].is_none() {
                        if only_one_idx.is_none() {
                            only_one_idx = Some(i);
                        } else {
                            only_one_idx = None;
                            break;
                        }
                    }
                }

                // If there was only one possibility then take it
                if let Some(idx) = only_one_idx {
                    ordering[idx] = Some(name.clone());
                    break;
                }
            }
        }

        ordering.into_iter().flatten().collect()
    }

    fn departure_product(&self, ordering: &[String]) -> u64 {
        ordering
            .iter()
            .zip(self.my_ticket.iter())
            .filter(|(o, _t)| o.starts_with("departure"))
            .map(|(_o, t)| *t as u64)
            .product()
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Notes {
    Notes::parser(input).unwrap().1
}

#[aoc(day16, part1)]
pub fn part1(input: &Notes) -> u32 {
    let error_rate = input.ticket_scanning_error_rate();
    assert_eq!(error_rate, 24021);
    error_rate
}

#[aoc(day16, part2)]
pub fn part2(input: &Notes) -> u64 {
    let ordering = input.find_field_ordering();
    let departure_product = input.departure_product(&ordering);
    assert_eq!(departure_product, 1289178686687);
    departure_product
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_ticket_scanning_error_rate() {
        let notes = input_generator(EXAMPLE_INPUT);
        assert_eq!(notes.ticket_scanning_error_rate(), 71);
    }

    #[test]
    fn test_find_field_ordering() {
        let notes = input_generator(EXAMPLE_INPUT);
        assert_eq!(
            notes.find_field_ordering(),
            ["row".to_owned(), "class".to_owned(), "seat".to_owned()]
        );
    }
}
