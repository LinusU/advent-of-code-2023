#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let mut sequences = vec![line];

            loop {
                let last = sequences.last().unwrap();
                let mut next = Vec::with_capacity(last.len() - 1);

                for i in 0..last.len() - 1 {
                    next.push(last[i + 1] - last[i]);
                }

                if next.iter().all(|&n| n == 0) {
                    break;
                }

                sequences.push(next);
            }

            sequences
                .into_iter()
                .map(|seq| *seq.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let mut sequences = vec![line];

            loop {
                let last = sequences.last().unwrap();
                let mut next = Vec::with_capacity(last.len() - 1);

                for i in 0..last.len() - 1 {
                    next.push(last[i + 1] - last[i]);
                }

                if next.iter().all(|&n| n == 0) {
                    break;
                }

                sequences.push(next);
            }

            let mut acc = 0;

            for seq in sequences.into_iter().rev() {
                acc = seq[0] - acc;
            }

            acc
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"),
        114
    );

    assert_eq!(
        part1("3 8 13 18 23 28 33 38 43 48 53 58 63 68 73 78 83 88 93 98 103"),
        108
    );

    assert_eq!(
        part1("-2 11 29 53 88 153 309 726 1835 4661 11532 27538 63421 141097 303903 635229 1292012 2564703 4984683 9517003 17906238"),
        33294945
    );

    assert_eq!(
        part1("0 -1 -2 -3 -4 -5 -6 -7 -8 -9 -10 -11 -12 -13 -14 -15 -16 -17 -18 -19 -20"),
        -21
    );

    assert_eq!(
        part1("-4 -9 -14 -19 -24 -29 -34 -39 -44 -49 -54 -59 -64 -69 -74 -79 -84 -89 -94 -99 -104"),
        -109
    );

    assert_eq!(
        part1("18 21 22 22 37 109 314 760 1558 2736 4047 4598 2207 -7605 -33043 -87863 -193462 -381037 -693732 -1189230 -1944591"),
        -3067851
    );

    assert_eq!(
        part1("0 -1 -2 -3 -4 -5 -6 -7 -8 -9 -10 -11 -12 -13 -14 -15 -16 -17 -18 -19 -20\n-4 -9 -14 -19 -24 -29 -34 -39 -44 -49 -54 -59 -64 -69 -74 -79 -84 -89 -94 -99 -104\n18 21 22 22 37 109 314 760 1558 2736 4047 4598 2207 -7605 -33043 -87863 -193462 -381037 -693732 -1189230 -1944591"),
        -3067851 - 109 - 21
    );
}

#[test]
fn test_part2() {
    assert_eq!(part2("10  13  16  21  30  45"), 5);
}
