use std::{collections::HashMap, str::FromStr};

trait Sorted {
    fn sorted(self) -> Self;
}

impl<T: Sized, const COUNT: usize> Sorted for [T; COUNT]
where
    T: Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

impl<T: Sized> Sorted for Vec<T>
where
    T: Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn with_jokers(self) -> Self {
        match self {
            Card::Jack => Card::Joker,
            card => card,
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand([Card; 5]);

impl Hand {
    fn kind(&self) -> HandKind {
        let mut counts = HashMap::new();

        for card in &self.0 {
            *counts.entry(card).or_insert(0usize) += 1;
        }

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let counts = counts.into_values().collect::<Vec<_>>().sorted();

        match (jokers, &counts[..]) {
            (4 | 5, _) => HandKind::FiveOfAKind,

            (0, [5]) => HandKind::FiveOfAKind,
            (1, [4]) => HandKind::FiveOfAKind,
            (2, [3]) => HandKind::FiveOfAKind,
            (3, [2]) => HandKind::FiveOfAKind,

            (0, [1, 4]) => HandKind::FourOfAKind,
            (1, [1, 3]) => HandKind::FourOfAKind,
            (2, [1, 2]) => HandKind::FourOfAKind,
            (3, [1, 1]) => HandKind::FourOfAKind,

            (0, [2, 3]) => HandKind::FullHouse,
            (1, [2, 2]) => HandKind::FullHouse,
            (3, _) => HandKind::FullHouse,

            (0, [1, 1, 3]) => HandKind::ThreeOfAKind,
            (1, [1, 1, 2]) => HandKind::ThreeOfAKind,
            (2, _) => HandKind::ThreeOfAKind,

            (0, [1, 2, 2]) => HandKind::TwoPair,

            (0, [1, 1, 1, 2]) => HandKind::OnePair,
            (1, _) => HandKind::OnePair,

            (0, [1, 1, 1, 1, 1]) => HandKind::HighCard,

            _ => unreachable!(),
        }
    }

    fn with_jokers(self) -> Self {
        Self(self.0.map(Card::with_jokers))
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(Card::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self(cards))
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let input = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| (hand.parse::<Hand>().unwrap(), bet.parse::<usize>().unwrap()))
        .map(|(hand, bet)| (hand.kind(), hand.0, bet))
        .collect::<Vec<_>>()
        .sorted();

    input
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, _, bet))| (rank + 1) * bet)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let input = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| (hand.parse::<Hand>().unwrap(), bet.parse::<usize>().unwrap()))
        .map(|(hand, bet)| (hand.with_jokers(), bet))
        .map(|(hand, bet)| (hand.kind(), hand.0, bet))
        .collect::<Vec<_>>()
        .sorted();

    input
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, _, bet))| (rank + 1) * bet)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"),
        6440
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"),
        5905
    );
}
