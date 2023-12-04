/*
    --- Day 2: Cube Conundrum ---
    You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

    The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

    As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

    To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

    You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

    For example, the record of a few games might look like this:

    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

    The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

    In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

    Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

    --- Part Two ---
    The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

    As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

    Again consider the example games from earlier:

    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
    Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
    Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
    Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
    The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

    For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let (left, right) = input.split_once(':').unwrap();
        let id = left.split_whitespace().last().unwrap().parse().unwrap();
        let sets: Vec<Set> = right.split(';').map(Set::from_str).collect();

        Self { id, sets }
    }

    fn max_set(&self) -> Set {
        let mut max = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in &self.sets {
            max.red = std::cmp::max(max.red, set.red);
            max.green = std::cmp::max(max.green, set.green);
            max.blue = std::cmp::max(max.blue, set.blue);
        }

        max
    }

    fn is_possible(&self, load: &Set) -> bool {
        let max = self.max_set();

        load.red >= max.red && load.green >= max.green && load.blue >= max.blue
    }
}

pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn from_str(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for s in input.split(',') {
            let (left, right) = s.trim().split_once(' ').unwrap();
            let n: u32 = left.parse().unwrap();
            match right {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                x => panic!("Invalid color: {x}"),
            }
        }

        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn check_load(games: &[Game]) -> u32 {
    let load = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    games
        .iter()
        .filter(|g| g.is_possible(&load))
        .map(|g| g.id)
        .sum()
}

fn power_sets(games: &[Game]) -> u32 {
    games.iter().map(|g| g.max_set().power()).sum()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(Game::from_str).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> u32 {
    let value = check_load(input);
    assert_eq!(value, 3099);
    value
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u32 {
    let value = power_sets(input);
    assert_eq!(value, 72970);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_check_load() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = check_load(&input);
        assert_eq!(value, 8);
    }

    #[test]
    fn test_power_sets() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = power_sets(&input);
        assert_eq!(value, 2286);
    }
}
