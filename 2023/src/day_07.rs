/*
    --- Day 7: Camel Cards ---
    Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

    "Did you bring the parts?"

    You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

    "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

    "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

    After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

    You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

    Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

    In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

    Every hand is exactly one type. From strongest to weakest, they are:

    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456
    Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

    If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

    So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

    To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

    So, the first step is to put the hands in order of strength:

    32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
    Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

    Find the rank of every hand in your set. What are the total winnings?

    --- Part Two ---
    To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

    To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

    J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

    Now, the above example goes very differently:

    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
    KK677 is now the only two pair, making it the second-weakest hand.
    T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
    With the new joker rule, the total winnings in this example are 5905.

    Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/

use std::cell::Cell;

use common::Mode;
use hashbag::HashBag;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid input: {value}"),
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: Cell<Option<HandType>>,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        if let Some(hand_type) = &self.hand_type.get() {
            return *hand_type;
        }

        let card_bag: HashBag<Card> = self.cards.iter().copied().collect();
        let mut card_counts: Vec<usize> = card_bag.set_iter().map(|(_card, count)| count).collect();
        card_counts.sort_by_key(|k| std::cmp::Reverse(*k));

        let hand_type = match (card_counts.as_slice(), card_bag.contains(&Card::Joker)) {
            ([5], _) => HandType::FiveOfKind,
            ([4, 1], 4) => HandType::FiveOfKind,
            ([4, 1], 1) => HandType::FiveOfKind,
            ([4, 1], 0) => HandType::FourOfKind,
            ([3, 2], 3) => HandType::FiveOfKind,
            ([3, 2], 2) => HandType::FiveOfKind,
            ([3, 2], 0) => HandType::FullHouse,
            ([3, 1, 1], 3) => HandType::FourOfKind,
            ([3, 1, 1], 1) => HandType::FourOfKind,
            ([3, 1, 1], 0) => HandType::ThreeOfKind,
            ([2, 2, 1], 2) => HandType::FourOfKind,
            ([2, 2, 1], 1) => HandType::FullHouse,
            ([2, 2, 1], 0) => HandType::TwoPair,
            ([2, 1, 1, 1], 2) => HandType::ThreeOfKind,
            ([2, 1, 1, 1], 1) => HandType::ThreeOfKind,
            ([2, 1, 1, 1], 0) => HandType::OnePair,
            ([1, 1, 1, 1, 1], 1) => HandType::OnePair,
            ([1, 1, 1, 1, 1], 0) => HandType::High,
            _ => panic!(
                "Unknown hand counts for hand {:?}: {card_counts:?}",
                self.cards
            ),
        };
        self.hand_type.set(Some(hand_type));
        hand_type
    }

    fn convert_j(&mut self) {
        for card in &mut self.cards {
            *card = match *card {
                Card::Jack => Card::Joker,
                x => x,
            }
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(' ').unwrap();
        let cards = left.chars().map(|c| c.into()).collect();
        let bid = right.parse().unwrap();

        Self {
            cards,
            bid,
            hand_type: Cell::new(None),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        for (me, you) in self.cards.iter().zip(other.cards.iter()) {
            match me.cmp(you) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn winnings(hands: &[Hand], mode: Mode) -> u64 {
    let mut hands = hands.to_vec();
    if mode == Mode::M2 {
        for hand in &mut hands {
            hand.convert_j();
        }
    };

    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid as u64 * (i + 1) as u64)
        .sum()
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Hand]) -> u64 {
    let value = winnings(input, Mode::M1);
    assert_eq!(value, 249204891);
    value
}

#[aoc(day7, part2)]
pub fn part2(input: &[Hand]) -> u64 {
    let value = winnings(input, Mode::M2);
    assert_eq!(value, 249666369);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_winnings_mode1() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = winnings(&input, Mode::M1);
        assert_eq!(value, 6440);
    }

    #[test]
    fn test_winnings_mode2() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = winnings(&input, Mode::M2);
        assert_eq!(value, 5905);
    }
}
