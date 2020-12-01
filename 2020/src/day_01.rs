/*
    --- Day 1: Report Repair ---
    After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

    The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

    To save your vacation, you need to get all fifty stars by December 25th.

    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

    Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

    Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

    For example, suppose your expense report contained the following:

    1721
    979
    366
    299
    675
    1456
    In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

    Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

    --- Part Two ---
    The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

    Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

    In your expense report, what is the product of the three entries that sum to 2020?
*/

fn find_sum2_2020(list: &[u32]) -> (u32, u32) {
    for &a in list {
        for &b in list {
            if a + b == 2020 {
                return (a, b);
            }
        }
    }
    panic!("No sum found!");
}

fn find_sum3_2020(list: &[u32]) -> (u32, u32, u32) {
    for &a in list {
        for &b in list {
            for &c in list {
                if a + b + c == 2020 {
                    return (a, b, c);
                }
            }
        }
    }
    panic!("No sum found!");
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let (a, b) = find_sum2_2020(input);
    let product = a * b;
    println!("Product of 2020: {}", product);
    assert_eq!(product, 436404);
    product
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let (a, b, c) = find_sum3_2020(input);
    let product = a * b * c;
    println!("Product of 2020: {}", product);
    assert_eq!(product, 274879808);
    product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_sum2_2020() {
        let input = "\
1721
979
366
299
675
1456";
        let expenses = input_generator(input);
        let (a, b) = find_sum2_2020(&expenses);
        assert_eq!(a, 1721);
        assert_eq!(b, 299);
        let product = a * b;
        assert_eq!(product, 514579);
    }

    #[test]
    fn test_find_sum3_2020() {
        let input = "\
1721
979
366
299
675
1456";
        let expenses = input_generator(input);
        let (a, b, c) = find_sum3_2020(&expenses);
        assert_eq!(a, 979);
        assert_eq!(b, 366);
        assert_eq!(c, 675);
        let product = a * b * c;
        assert_eq!(product, 241861950);
    }
}
