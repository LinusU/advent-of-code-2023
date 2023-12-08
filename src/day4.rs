use std::{collections::HashSet, str::FromStr};

struct Card {
    our_numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Card {
    fn points(&self) -> u64 {
        match self.our_numbers.intersection(&self.winning_numbers).count() {
            0 => 0,
            n => 2u64.pow((n as u32) - 1),
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card ").unwrap();

        let (_id, s) = s.split_once(':').unwrap();

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

#[test]
fn test_part1() {
    assert_eq!(part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 13);
}
