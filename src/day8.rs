use std::{
    collections::HashMap,
    fmt::{self, Debug},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId([u8; 3]);

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
