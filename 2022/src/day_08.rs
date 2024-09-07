/*
    --- Day 8: Treetop Tree House ---
    The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

    First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

    The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

    30373
    25512
    65332
    33549
    35390
    Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

    A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

    All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
    With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

    Consider your map; how many trees are visible from outside the grid?

    --- Part Two ---
    Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

    To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

    The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

    In the example above, consider the middle 5 in the second row:

    30373
    25512
    65332
    33549
    35390
    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
    A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

    However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

    30373
    25512
    65332
    33549
    35390
    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
    This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

    Consider each tree on your map. What is the highest scenic score possible for any tree?
*/

use std::str::FromStr;

use itertools::Itertools;

pub struct Trees {
    data: Vec<Vec<u8>>,
}

impl FromStr for Trees {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
            .collect();

        Ok(Self { data })
    }
}

impl Trees {
    fn count_visible(&self) -> u64 {
        let mut is_vis: Vec<Vec<bool>> = vec![vec![false; self.data[0].len()]; self.data.len()];
        let mut biggest;

        #[expect(clippy::needless_range_loop)]
        // False alarm, we really are using this to index multiple structures
        for row in 0..self.data.len() {
            // Left to right
            biggest = None;
            for col in 0..self.data[0].len() {
                let curr = self.data[row][col];
                is_vis[row][col] = is_vis[row][col] || biggest.map_or(true, |big| curr > big);
                biggest = Some(biggest.map_or(curr, |big| big.max(curr)));
            }

            // Right to left
            biggest = None;
            for col in (0..self.data[0].len()).rev() {
                let curr = self.data[row][col];
                is_vis[row][col] = is_vis[row][col] || biggest.map_or(true, |big| curr > big);
                biggest = Some(biggest.map_or(curr, |big| big.max(curr)));
            }
        }

        #[expect(clippy::needless_range_loop)]
        // False alarm, we really are using this to index multiple structures
        for col in 0..self.data[0].len() {
            // Left to right
            biggest = None;
            for row in 0..self.data.len() {
                let curr = self.data[row][col];
                is_vis[row][col] = is_vis[row][col] || biggest.map_or(true, |big| curr > big);
                biggest = Some(biggest.map_or(curr, |big| big.max(curr)));
            }

            // Right to left
            biggest = None;
            for row in (0..self.data.len()).rev() {
                let curr = self.data[row][col];
                is_vis[row][col] = is_vis[row][col] || biggest.map_or(true, |big| curr > big);
                biggest = Some(biggest.map_or(curr, |big| big.max(curr)));
            }
        }

        is_vis.into_iter().flatten().filter(|vis| *vis).count() as u64
    }

    fn scenic_score(&self, row: usize, col: usize) -> u64 {
        let my_size = self.data[row][col];

        let mut proceed = true;
        let up = (0..col)
            .rev()
            .take_while(|other_col| {
                proceed && {
                    proceed = self.data[row][*other_col] < my_size;
                    true
                }
            })
            .count();
        proceed = true;
        let down = (col + 1..self.data[0].len())
            .take_while(|other_col| {
                proceed && {
                    proceed = self.data[row][*other_col] < my_size;
                    true
                }
            })
            .count();
        proceed = true;
        let left = (0..row)
            .rev()
            .take_while(|other_row| {
                proceed && {
                    proceed = self.data[*other_row][col] < my_size;
                    true
                }
            })
            .count();
        proceed = true;
        let right = (row + 1..self.data.len())
            .take_while(|other_row| {
                proceed && {
                    proceed = self.data[*other_row][col] < my_size;
                    true
                }
            })
            .count();

        (up * down * left * right) as u64
    }

    fn best_scenic_score(&self) -> u64 {
        (0..self.data.len())
            .cartesian_product(0..self.data[0].len())
            .map(|(row, col)| self.scenic_score(row, col))
            .max()
            .unwrap()
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Trees {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
pub fn part1(input: &Trees) -> u64 {
    let visible = input.count_visible();
    assert_eq!(visible, 1700);
    visible
}

#[aoc(day8, part2)]
pub fn part2(input: &Trees) -> u64 {
    let score = input.best_scenic_score();
    assert_eq!(score, 470596);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_count_visible() {
        let input = input_generator(EXAMPLE_INPUT);
        let count = input.count_visible();
        assert_eq!(count, 21);
    }

    #[test]
    fn test_scenic_score() {
        let input = input_generator(EXAMPLE_INPUT);

        let score = input.scenic_score(1, 2);
        assert_eq!(score, 4);

        let score = input.scenic_score(3, 2);
        assert_eq!(score, 8);
    }

    #[test]
    fn test_best_scenic_score() {
        let input = input_generator(EXAMPLE_INPUT);
        let score = input.best_scenic_score();
        assert_eq!(score, 8);
    }
}
