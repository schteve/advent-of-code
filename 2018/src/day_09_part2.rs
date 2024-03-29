/*
    --- Part Two ---
    Amused by the speed of your answer, the Elves are curious:

    What would the new winning Elf's score be if the number of the last marble were 100 times larger?
*/

use common::LinkedListCirc;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, pair},
    IResult,
};

struct Game {
    max_player: u32,
    max_marble: u32,
    state: LinkedListCirc<u32>,
    current_player: u32,
    player_score: Vec<u32>,
}

impl Game {
    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (max_player, max_marble)) = pair(
            map_res(digit1, |p: &str| p.parse::<u32>()),
            delimited(
                tag(" players; last marble is worth "),
                map_res(digit1, |m: &str| m.parse::<u32>()),
                tag(" points"),
            ),
        )(input)?;

        let mut state = LinkedListCirc::new();
        state.insert(0, 0);

        Ok((
            input,
            Self {
                max_player,
                max_marble,
                state,
                current_player: 0,
                player_score: vec![0; max_player as usize],
            },
        ))
    }

    fn place_marble(&mut self, value: u32, offset: i32) {
        self.state.insert_from_last_op(value, offset);
    }

    fn remove_marble(&mut self, offset: i32) -> u32 {
        self.state.remove_from_last_op(offset)
    }

    fn play(&mut self) -> u32 {
        // Assumes player scores are initialized to 0
        for marble_number in 1..=self.max_marble {
            // Marble 0 is placed at initialization
            if marble_number % 23 == 0 {
                // Don't place this marble, remove the one 7 to the left, and add both to the current player's score.
                let removed = self.state.remove_from_last_op(-7);
                self.player_score[self.current_player as usize] += marble_number + removed;
            } else {
                // Place this marble 2 to the right
                self.state.insert_from_last_op(marble_number, 2);
            }

            self.current_player += 1;
            if self.current_player >= self.max_player {
                self.current_player = 0;
            }
        }

        *self.player_score.iter().max().unwrap()
    }
}

#[aoc(day9, part2)]
pub fn solve(input: &str) -> u32 {
    let mut game = Game::from_string(input);
    game.max_marble *= 100;
    let high_score = game.play();
    println!("High score: {}", high_score);
    assert_eq!(high_score, 3009951158);
    high_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_place_marble() {
        let input = "9 players; last marble is worth 25 points";
        let mut game = Game::from_string(input);
        game.place_marble(1, 2);
        assert_eq!(game.state.to_vec(), vec![0, 1]);
        game.place_marble(2, 2);
        assert_eq!(game.state.to_vec(), vec![0, 2, 1]);
        game.place_marble(3, 2);
        assert_eq!(game.state.to_vec(), vec![0, 2, 1, 3]);
        game.place_marble(4, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 1, 3]);
        game.place_marble(5, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 3]);
        game.place_marble(6, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 6, 3]);
        game.place_marble(7, 2);
        assert_eq!(game.state.to_vec(), vec![0, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(8, 2);
        assert_eq!(game.state.to_vec(), vec![0, 8, 4, 2, 5, 1, 6, 3, 7]);
        game.place_marble(9, 2);
        assert_eq!(game.state.to_vec(), vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7]);
    }

    #[test]
    fn test_play() {
        let input = "9 players; last marble is worth 25 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 32);
        assert_eq!(
            game.state.to_vec(),
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ]
        );

        let input = "10 players; last marble is worth 1618 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 8317);

        let input = "13 players; last marble is worth 7999 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 146373);

        let input = "17 players; last marble is worth 1104 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 2764);

        let input = "21 players; last marble is worth 6111 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 54718);

        let input = "30 players; last marble is worth 5807 points";
        let mut game = Game::from_string(input);
        let high_score = game.play();
        assert_eq!(high_score, 37305);
    }
}
