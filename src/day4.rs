use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Card {
    id: u64,
    our_numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Card {
    fn matching_numbers(&self) -> usize {
        self.our_numbers.intersection(&self.winning_numbers).count()
    }

    fn points(&self) -> u64 {
        match self.matching_numbers() {
            0 => 0,
            n => 2u64.pow((n as u32) - 1),
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card").unwrap();

        let (id, s) = s.split_once(':').unwrap();

        let id = id.trim().parse::<u64>().unwrap();
        let s = s.trim();

        let (winning_numbers, our_numbers) = s.split_once('|').unwrap();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<HashSet<_>>();

        let our_numbers = our_numbers
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<HashSet<_>>();

        Ok(Card {
            id,
            our_numbers,
            winning_numbers,
        })
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .map(|line| line.parse::<Card>().unwrap())
        .map(|card| card.points())
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut card_copies = HashMap::<u64, usize>::new();
    let mut card_originals = 0;

    for line in input.split('\n') {
        let card = line.parse::<Card>().unwrap();

        let number_of_copies = *card_copies.entry(card.id).or_insert(0);
        let matching_numbers = card.matching_numbers();

        card_originals += 1;

        for i in 1..=matching_numbers {
            *card_copies.entry(card.id + i as u64).or_insert(0) += 1 + number_of_copies;
        }
    }

    card_copies.into_values().sum::<usize>() + card_originals
}

#[test]
fn test_part1() {
    assert_eq!(part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
}
