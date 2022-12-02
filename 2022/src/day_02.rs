/*
    --- Day 2: Rock Paper Scissors ---
    The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

    Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

    Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

    The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

    The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

    Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

    For example, suppose you were given the following strategy guide:

    A Y
    B X
    C Z
    This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
    In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

    What would your total score be if everything goes exactly according to your strategy guide?

    The first half of this puzzle is complete! It provides one gold star: *

    --- Part Two ---
    The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

    The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
    Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

    Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/

#[derive(Clone, Copy)]
pub enum Theirs {
    A,
    B,
    C,
}

#[derive(Clone, Copy)]
pub enum Mine {
    X,
    Y,
    Z,
}

enum Wld {
    Win,
    Lose,
    Draw,
}

impl Wld {
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub struct Strategy {
    theirs: Theirs,
    mine: Mine,
}

impl Strategy {
    fn from_str(input: &str) -> Self {
        let mut it = input.trim().split_ascii_whitespace();
        let theirs = match it.next().unwrap() {
            "A" => Theirs::A,
            "B" => Theirs::B,
            "C" => Theirs::C,
            x => panic!("Bad theirs: {}", x),
        };
        let mine = match it.next().unwrap() {
            "X" => Mine::X,
            "Y" => Mine::Y,
            "Z" => Mine::Z,
            x => panic!("Bad mine: {}", x),
        };
        Self { theirs, mine }
    }

    fn wld_from_shape(&self) -> Wld {
        match (self.theirs, self.mine) {
            (Theirs::A, Mine::X) => Wld::Draw,
            (Theirs::A, Mine::Y) => Wld::Win,
            (Theirs::A, Mine::Z) => Wld::Lose,
            (Theirs::B, Mine::X) => Wld::Lose,
            (Theirs::B, Mine::Y) => Wld::Draw,
            (Theirs::B, Mine::Z) => Wld::Win,
            (Theirs::C, Mine::X) => Wld::Win,
            (Theirs::C, Mine::Y) => Wld::Lose,
            (Theirs::C, Mine::Z) => Wld::Draw,
        }
    }

    fn shape_from_mine(&self) -> Shape {
        match self.mine {
            Mine::X => Shape::Rock,
            Mine::Y => Shape::Paper,
            Mine::Z => Shape::Scissors,
        }
    }

    fn wld_from_mine(&self) -> Wld {
        match self.mine {
            Mine::X => Wld::Lose,
            Mine::Y => Wld::Draw,
            Mine::Z => Wld::Win,
        }
    }

    fn shape_from_wld(&self) -> Shape {
        match (self.theirs, self.mine) {
            (Theirs::A, Mine::X) => Shape::Scissors,
            (Theirs::A, Mine::Y) => Shape::Rock,
            (Theirs::A, Mine::Z) => Shape::Paper,
            (Theirs::B, Mine::X) => Shape::Rock,
            (Theirs::B, Mine::Y) => Shape::Paper,
            (Theirs::B, Mine::Z) => Shape::Scissors,
            (Theirs::C, Mine::X) => Shape::Paper,
            (Theirs::C, Mine::Y) => Shape::Scissors,
            (Theirs::C, Mine::Z) => Shape::Rock,
        }
    }
}

fn score1(strats: &[Strategy]) -> u64 {
    strats
        .iter()
        .map(|s| {
            let wld = s.wld_from_shape();
            let shape = s.shape_from_mine();
            wld.score() + shape.score()
        })
        .sum()
}

fn score2(strats: &[Strategy]) -> u64 {
    strats
        .iter()
        .map(|s| {
            let wld = s.wld_from_mine();
            let shape = s.shape_from_wld();
            wld.score() + shape.score()
        })
        .sum()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Strategy> {
    input.lines().map(Strategy::from_str).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Strategy]) -> u64 {
    let score = score1(input);
    assert_eq!(score, 11475);
    score
}

#[aoc(day2, part2)]
pub fn part2(input: &[Strategy]) -> u64 {
    let score = score2(input);
    assert_eq!(score, 16862);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_score1() {
        let input = input_generator(EXAMPLE_INPUT);
        let score = score1(&input);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_score2() {
        let input = input_generator(EXAMPLE_INPUT);
        let score = score2(&input);
        assert_eq!(score, 12);
    }
}
