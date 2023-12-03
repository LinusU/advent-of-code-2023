use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cube {
    Blue,
    Green,
    Red,
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Cube::Blue),
            "green" => Ok(Cube::Green),
            "red" => Ok(Cube::Red),
            _ => Err(format!("Invalid cube color: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<HashMap<Cube, u64>>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").unwrap();
        let (id, s) = s.split_once(':').unwrap();

        let id = id.parse::<u64>().unwrap();
        let s = s.trim();

        let mut sets = Vec::new();

        for s in s.split(';') {
            let mut set = HashMap::new();

            for s in s.split(',') {
                let (n, cube) = s.trim().split_once(' ').unwrap();

                let n = n.parse::<u64>().unwrap();
                let cube = cube.parse::<Cube>().unwrap();

                set.insert(cube, n);
            }

            sets.push(set);
        }

        Ok(Game { id, sets })
    }
}

impl Game {
    fn minimum_cubes_needed(&self, _type: Cube) -> u64 {
        let mut min = 0;

        for set in &self.sets {
            if let Some(n) = set.get(&_type) {
                min = min.max(*n);
            }
        }

        min
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .filter_map(|line| {
            let game = line.parse::<Game>().unwrap();

            if game.minimum_cubes_needed(Cube::Blue) > 14 {
                return None;
            }

            if game.minimum_cubes_needed(Cube::Green) > 13 {
                return None;
            }

            if game.minimum_cubes_needed(Cube::Red) > 12 {
                return None;
            }

            Some(game.id)
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8);
}
