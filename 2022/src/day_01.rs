/*
    --- Day 1: Calorie Counting ---
    Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

    To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

    The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

    The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

    For example, suppose the Elves finish writing their items' Calories and end up with the following list:

    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    This list represents the Calories of the food carried by five Elves:

    The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    The second Elf is carrying one food item with 4000 Calories.
    The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    The fifth Elf is carrying one food item with 10000 Calories.
    In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

    Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

    --- Part Two ---
    By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

    To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

    In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

    Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

fn calc_totals(elves: &[Vec<u64>]) -> Vec<u64> {
    let mut totals: Vec<u64> = elves.iter().map(|elf| elf.iter().sum()).collect();
    totals.sort_unstable();
    totals
}

fn find_most_calories(elves: &[Vec<u64>]) -> u64 {
    let totals = calc_totals(elves);
    *totals.last().unwrap()
}

fn find_top3_calories(elves: &[Vec<u64>]) -> u64 {
    let mut totals = calc_totals(elves);
    (0..3).map(|_| totals.pop().unwrap()).sum()
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse::<u64>().unwrap())
        }
    }
    if !elf.is_empty() {
        elves.push(elf);
    }
    elves
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u64>]) -> u64 {
    let most = find_most_calories(input);
    assert_eq!(most, 67633);
    most
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u64>]) -> u64 {
    let top3 = find_top3_calories(input);
    assert_eq!(top3, 199628);
    top3
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_find_most_calories() {
        let input = input_generator(EXAMPLE_INPUT);
        let most = find_most_calories(&input);
        assert_eq!(most, 24000);
    }

    #[test]
    fn test_find_top3_calories() {
        let input = input_generator(EXAMPLE_INPUT);
        let top3 = find_top3_calories(&input);
        assert_eq!(top3, 45000);
    }
}