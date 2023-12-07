use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

pub fn part_one(input: &str) -> usize {
    compute(input, Card::Card)
}

pub fn part_two(input: &str) -> usize {
    compute(input, Card::Wildcard)
}

fn compute(input: &str, char_to_card: impl Fn(char) -> Card) -> usize {
    let mut hands = input
        .lines()
        .map(|line| parse_line(line, &char_to_card))
        .collect::<Vec<_>>();
    hands.sort_by(|(left, _), (right, _)| left.cmp(right));
    hands
        .iter()
        .enumerate()
        .map(|(index, (_, rank))| (index + 1) * rank)
        .sum()
}

fn parse_line(line: &str, char_to_card: impl Fn(char) -> Card) -> (Hand, usize) {
    let mut columns = line.split_whitespace();
    let cards = columns
        .next()
        .unwrap()
        .chars()
        .fold(Vec::new(), |mut acc, ch| {
            acc.push(char_to_card(ch));
            acc
        });
    let bid = columns.next().unwrap().parse::<usize>().unwrap();
    (Hand(cards), bid)
}

const BASIC_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const WILDCARD_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Card {
    Card(char),
    Wildcard(char),
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let (&ch, &other_ch, order) = match (self, other) {
            (Card::Card(ch), Card::Card(other_ch)) => (ch, other_ch, BASIC_ORDER),
            (Card::Wildcard(ch), Card::Wildcard(other_ch)) => (ch, other_ch, WILDCARD_ORDER),
            _ => unreachable!(),
        };
        let this_pos = order.iter().position(|&c| c == ch).unwrap();
        let that_pos = order.iter().position(|&c| c == other_ch).unwrap();
        this_pos.cmp(&that_pos)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Hand(Vec<Card>);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_rank = apply_rules(self);
        let other_rank = apply_rules(other);
        match this_rank.cmp(&other_rank) {
            Ordering::Equal => self
                .0
                .iter()
                .zip(&other.0)
                .find(|(left, right)| left != right)
                .map(|(left, right)| left.cmp(right))
                .unwrap(),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const J: Card = Card::Wildcard('J');

const FIVE_OF_A_KIND: usize = 6;
const FOUR_OF_A_KIND: usize = 5;
const FULL_HOUSE: usize = 4;
const THREE_OF_A_KIND: usize = 3;
const TWO_PAIR: usize = 2;
const ONE_PAIR: usize = 1;
const HIGH_CARD: usize = 0;

fn apply_rules(hand: &Hand) -> usize {
    let mut mapped = map_cards(&hand.0);
    mapped.sort_by_key(|hand| Reverse(hand.1));
    mapped
        .iter()
        .find(|(&card, _)| card == J)
        .map(|_| match mapped.as_slice() {
            [(_, _)] | [(_, _), (_, _)] => FIVE_OF_A_KIND,
            [(_, 3), ..] => FOUR_OF_A_KIND,
            [(_, 2), (_, 2), (&z, 1)] if z != J => FOUR_OF_A_KIND,
            [(_, 2), (_, 2), (_, 1)] => FULL_HOUSE,
            [(_, 2), (_, 1), (_, 1), (_, 1)] => THREE_OF_A_KIND,
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => ONE_PAIR,
            _ => HIGH_CARD,
        })
        .unwrap_or(match mapped.as_slice() {
            [(_, _)] => FIVE_OF_A_KIND,
            [(_, 4), (_, _)] => FOUR_OF_A_KIND,
            [(_, 3), (_, 2)] => FULL_HOUSE,
            [(_, 3), (_, 1), (_, 1)] => THREE_OF_A_KIND,
            [(_, 2), (_, 2), ..] => TWO_PAIR,
            [(_, 2), ..] => ONE_PAIR,
            _ => HIGH_CARD,
        })
}

fn map_cards(cards: &[Card]) -> Vec<(&Card, usize)> {
    cards
        .iter()
        .fold(HashMap::new(), |mut acc, card| {
            acc.entry(card).and_modify(|count| *count += 1).or_insert(1);
            acc
        })
        .into_iter()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        let result = part_one(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../resources/day7_part1");
        let result = part_one(input);
        assert_eq!(result, 250898830);
    }

    #[test]
    fn part_two_example1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        let result = part_two(input);
        assert_eq!(result, 5905);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../resources/day7_part1");
        let result = part_two(input);
        assert_eq!(result, 252127335);
    }

    #[test]
    fn test_ordering() {
        let k = Card::Card('K');
        let j = Card::Card('J');
        assert_eq!(k.partial_cmp(&j), Some(Ordering::Greater))
    }

    #[test]
    fn should_say_hand_is_greater() {
        let left = Hand(vec![
            Card::Card('Q'),
            Card::Card('Q'),
            Card::Card('Q'),
            Card::Card('J'),
            Card::Card('A'),
        ]);
        let right = Hand(vec![
            Card::Card('T'),
            Card::Card('5'),
            Card::Card('5'),
            Card::Card('J'),
            Card::Card('5'),
        ]);
        assert_eq!(left.cmp(&right), Ordering::Greater)
    }

    #[test]
    fn should_say_hand_is_less() {
        let left = Hand(vec![
            Card::Card('K'),
            Card::Card('T'),
            Card::Card('J'),
            Card::Card('J'),
            Card::Card('T'),
        ]);
        let right = Hand(vec![
            Card::Card('T'),
            Card::Card('5'),
            Card::Card('5'),
            Card::Card('J'),
            Card::Card('5'),
        ]);
        assert_eq!(left.cmp(&right), Ordering::Less)
    }

    #[test]
    fn should_say_card_is_greater() {
        let left = Card::Card('A');
        let right = Card::Card('2');
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn should_give_correct_rank_with_wildcard_rules() {
        let hand = "JJQQQ";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 6);
    }

    #[test]
    fn should_be_full_house_with_wildcard_rules() {
        let hand = "33QQQ";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 4);
    }

    #[test]
    fn should_be_three_of_a_kind_with_wildcard_rules() {
        let hand = "3KJJQ";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 3);
    }

    #[test]
    fn should_be_two_pair_with_wildcard_rules() {
        let hand = "3KQQK";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 2);
    }

    #[test]
    fn should_be_one_pair_with_wildcard_rules() {
        let hand = "J4729";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 1);
    }

    #[test]
    fn should_be_high_card_with_wildcard_rules() {
        let hand = "3KQT1";
        let hand = Hand(hand.chars().map(Card::Wildcard).collect());
        assert_eq!(apply_rules(&hand), 0);
    }
}
