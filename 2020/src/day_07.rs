/*
    --- Day 7: Handy Haversacks ---
    You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

    Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

    For example, consider the following rules:

    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.
    These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

    You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

    In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

    How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

    --- Part Two ---
    It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

    Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
    So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

    Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

    Here's another example:

    shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.
    In this example, a single shiny gold bag must contain 126 other bags.

    How many individual bags are required inside your single shiny gold bag?
*/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, recognize, success},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Ingredient {
    num: u32,
    color: String,
}

impl Ingredient {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (num, color)) = pair(
            terminated(map_res(digit1, |x: &str| x.parse::<u32>()), char(' ')),
            map(recognize(tuple((alpha1, char(' '), alpha1))), |c: &str| {
                c.to_owned()
            }),
        )(input)?;

        Ok((input, Self { num, color }))
    }
}

struct Recipe {
    color: String,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (color, ingredients)) = pair(
            map(
                delimited(
                    multispace0,
                    recognize(tuple((alpha1, char(' '), alpha1))),
                    tag(" bags contain "),
                ),
                |p: &str| p.to_owned(),
            ),
            terminated(
                alt((
                    separated_list1(alt((tag(" bag, "), tag(" bags, "))), Ingredient::parser),
                    terminated(success(Vec::new()), tag("no other")),
                )),
                alt((tag(" bag."), tag(" bags."))),
            ),
        )(input)?;

        Ok((input, Self { color, ingredients }))
    }
}

pub struct BagCookBook {
    recipes_map: HashMap<String, Vec<Ingredient>>,
}

impl BagCookBook {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, recipes) = many1(Recipe::parser)(input)?;

        let recipes_map = recipes
            .into_iter()
            .map(|recipe| (recipe.color, recipe.ingredients))
            .collect();
        Ok((input, Self { recipes_map }))
    }

    fn gen_reverse_lookup(&self) -> HashMap<String, Vec<String>> {
        let mut reverse_lookup: HashMap<String, Vec<String>> = HashMap::new();
        for (k, v) in self.recipes_map.iter() {
            for ingredient in v {
                let e = reverse_lookup
                    .entry(ingredient.color.clone())
                    .or_insert(Vec::new());
                e.push(k.clone());
            }
        }

        // Sort the vectors so it's easy to check later
        for (_s, v) in reverse_lookup.iter_mut() {
            v.sort_unstable();
        }

        reverse_lookup
    }

    fn count_contains_gold(&self) -> usize {
        let mut contains_gold: HashSet<String> = HashSet::new();
        let mut frontier: Vec<String> = vec!["shiny gold".into()];

        let reverse_lookup = self.gen_reverse_lookup();
        while let Some(curr_bag) = frontier.pop() {
            if let Some(parents) = reverse_lookup.get(&curr_bag) {
                for p in parents {
                    if contains_gold.insert(p.clone()) == true {
                        // New value; add it to the frontier
                        frontier.push(p.clone());
                    }
                }
            }
        }

        contains_gold.len()
    }

    fn count_bags_in_gold(&self) -> u32 {
        let mut totals: HashMap<String, u32> = HashMap::new();
        let mut frontier: Vec<(String, u32)> = vec![("shiny gold".into(), 1)];
        while let Some((bag, count)) = frontier.pop() {
            if let Some(bag_contents) = self.recipes_map.get(&bag) {
                for ingredient in bag_contents {
                    if let Some(pos) = frontier.iter_mut().position(|x| x.0 == ingredient.color) {
                        frontier[pos] = (
                            frontier[pos].0.clone(),
                            frontier[pos].1 + count * ingredient.num,
                        );
                    } else {
                        frontier.push((ingredient.color.clone(), count * ingredient.num));
                    }
                }
            }

            let total_count = totals.entry(bag).or_insert(0);
            *total_count += count;
        }

        totals.values().sum::<u32>() - 1 // Subtract one since we're not supposed to count the original gold
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> BagCookBook {
    BagCookBook::parser(input).unwrap().1
}

#[aoc(day7, part1)]
pub fn part1(input: &BagCookBook) -> usize {
    let count = input.count_contains_gold();
    println!("Bags containing shiny gold: {}", count);
    assert_eq!(count, 372);
    count
}

#[aoc(day7, part2)]
pub fn part2(input: &BagCookBook) -> u32 {
    let count = input.count_bags_in_gold();
    println!("Bags contained by a shiny gold: {}", count);
    assert_eq!(count, 8015);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    static EXAMPLE_INPUT2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_input_generator() {
        let cookbook = input_generator(EXAMPLE_INPUT1);
        let expected: Vec<(String, Vec<Ingredient>)> = vec![
            (
                "light red".into(),
                vec![
                    Ingredient {
                        num: 1,
                        color: "bright white".into(),
                    },
                    Ingredient {
                        num: 2,
                        color: "muted yellow".into(),
                    },
                ],
            ),
            (
                "dark orange".into(),
                vec![
                    Ingredient {
                        num: 3,
                        color: "bright white".into(),
                    },
                    Ingredient {
                        num: 4,
                        color: "muted yellow".into(),
                    },
                ],
            ),
            (
                "bright white".into(),
                vec![Ingredient {
                    num: 1,
                    color: "shiny gold".into(),
                }],
            ),
            (
                "muted yellow".into(),
                vec![
                    Ingredient {
                        num: 2,
                        color: "shiny gold".into(),
                    },
                    Ingredient {
                        num: 9,
                        color: "faded blue".into(),
                    },
                ],
            ),
            (
                "shiny gold".into(),
                vec![
                    Ingredient {
                        num: 1,
                        color: "dark olive".into(),
                    },
                    Ingredient {
                        num: 2,
                        color: "vibrant plum".into(),
                    },
                ],
            ),
            (
                "dark olive".into(),
                vec![
                    Ingredient {
                        num: 3,
                        color: "faded blue".into(),
                    },
                    Ingredient {
                        num: 4,
                        color: "dotted black".into(),
                    },
                ],
            ),
            (
                "vibrant plum".into(),
                vec![
                    Ingredient {
                        num: 5,
                        color: "faded blue".into(),
                    },
                    Ingredient {
                        num: 6,
                        color: "dotted black".into(),
                    },
                ],
            ),
            ("faded blue".into(), vec![]),
            ("dotted black".into(), vec![]),
        ];
        assert_eq!(cookbook.recipes_map, expected.into_iter().collect());
    }

    #[test]
    fn test_gen_reverse_lookup() {
        let cookbook = input_generator(EXAMPLE_INPUT1);
        let reverse_lookup = cookbook.gen_reverse_lookup();
        let expected: Vec<(String, Vec<String>)> = vec![
            (
                "bright white".into(),
                vec!["dark orange".into(), "light red".into()],
            ),
            (
                "muted yellow".into(),
                vec!["dark orange".into(), "light red".into()],
            ),
            (
                "shiny gold".into(),
                vec!["bright white".into(), "muted yellow".into()],
            ),
            (
                "faded blue".into(),
                vec![
                    "dark olive".into(),
                    "muted yellow".into(),
                    "vibrant plum".into(),
                ],
            ),
            ("dark olive".into(), vec!["shiny gold".into()]),
            ("vibrant plum".into(), vec!["shiny gold".into()]),
            (
                "dotted black".into(),
                vec!["dark olive".into(), "vibrant plum".into()],
            ),
        ];
        assert_eq!(reverse_lookup, expected.into_iter().collect());
    }

    #[test]
    fn test_count_contains_gold() {
        let cookbook = input_generator(EXAMPLE_INPUT1);
        assert_eq!(cookbook.count_contains_gold(), 4);
    }

    #[test]
    fn test_count_bags_in_gold() {
        let cookbook = input_generator(EXAMPLE_INPUT1);
        assert_eq!(cookbook.count_bags_in_gold(), 32);

        let cookbook = input_generator(EXAMPLE_INPUT2);
        assert_eq!(cookbook.count_bags_in_gold(), 126);
    }
}
