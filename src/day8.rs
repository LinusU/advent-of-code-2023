use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    fmt::{self, Debug},
    str::FromStr,
};

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // Binary GCD algorithm
    let mut d = 0;

    loop {
        if a == b {
            return a * 2usize.pow(d);
        }

        if a.is_even() && b.is_even() {
            d += 1;
            a /= 2;
            b /= 2;
            continue;
        }

        if a.is_even() {
            a /= 2;
            continue;
        }

        if b.is_even() {
            b /= 2;
            continue;
        }

        if a < b {
            (a, b) = (b, a);
        }

        let c = a - b;
        a = c / 2;
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(48, 18), 6);
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(48, 18), 144);
    assert_eq!(lcm(14_681, 20_221), 1_071_713);
    assert_eq!(lcm(1_071_713, 21_883), 84_665_327);
    assert_eq!(lcm(84_665_327, 16_343), 4_995_254_293);
    assert_eq!(lcm(4_995_254_293, 13_019), 234_776_951_771);
    assert_eq!(lcm(234_776_951_771, 16_897), 14_321_394_058_031);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId([u8; 3]);

impl NodeId {
    fn is_ghost_start(&self) -> bool {
        self.0[2] == b'A'
    }

    fn is_ghost_end(&self) -> bool {
        self.0[2] == b'Z'
    }
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl FromStr for NodeId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeId(s.as_bytes().try_into().unwrap()))
    }
}

#[derive(Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let (instructions, input) = input.split_once("\n\n").unwrap();

    let mut network = HashMap::new();

    for line in input.lines() {
        let (id, line) = line.split_once('=').unwrap();

        let id = id.trim().parse::<NodeId>().unwrap();

        let (left, right) = line.split_once(',').unwrap();

        let left = left.trim().strip_prefix('(').unwrap();
        let right = right.trim().strip_suffix(')').unwrap();

        let left = left.parse::<NodeId>().unwrap();
        let right = right.parse::<NodeId>().unwrap();

        network.insert(id, Node { left, right });
    }

    let target = NodeId(*b"ZZZ");

    let mut current = NodeId(*b"AAA");
    let mut steps = 0;
    let mut instructions = instructions.chars().cycle();

    while current != target {
        let node = network.get(&current).unwrap();

        match instructions.next() {
            Some('L') => current = node.left,
            Some('R') => current = node.right,
            _ => unreachable!(),
        }

        steps += 1;
    }

    steps
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let (instructions, input) = input.split_once("\n\n").unwrap();

    let mut network = HashMap::new();

    for line in input.lines() {
        let (id, line) = line.split_once('=').unwrap();

        let id = id.trim().parse::<NodeId>().unwrap();

        let (left, right) = line.split_once(',').unwrap();

        let left = left.trim().strip_prefix('(').unwrap();
        let right = right.trim().strip_suffix(')').unwrap();

        let left = left.parse::<NodeId>().unwrap();
        let right = right.parse::<NodeId>().unwrap();

        network.insert(id, Node { left, right });
    }

    network
        .keys()
        .filter(|id| id.is_ghost_start())
        .map(|ghost| {
            let mut steps = 0usize;
            let mut instructions = instructions.chars().enumerate().cycle();

            let mut current = *ghost;
            let mut seen = HashMap::new();

            let mut ends = Vec::new();

            loop {
                let node = network.get(&current).unwrap();
                let (idx, instruction) = instructions.next().unwrap();

                match instruction {
                    'L' => current = node.left,
                    'R' => current = node.right,
                    _ => unreachable!(),
                }

                steps += 1;

                if current.is_ghost_end() {
                    ends.push(steps);
                }

                match seen.entry((idx, current)) {
                    Vacant(e) => {
                        e.insert(steps);
                    }
                    Occupied(e) => {
                        assert_eq!(ends.len(), 1);

                        let steps_until_start_of_cycle = *e.get();
                        let steps_until_exit = ends[0];
                        let cycle_length = steps - steps_until_start_of_cycle;

                        assert_eq!(steps_until_exit, cycle_length);

                        return steps_until_exit;
                    }
                }
            }
        })
        .reduce(lcm)
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)"),
        2,
    );

    assert_eq!(
        part1("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)"),
        6,
    );
}
