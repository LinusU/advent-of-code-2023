#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times.strip_prefix("Time:").unwrap();
    let distances = distances.strip_prefix("Distance:").unwrap();

    let times = times
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = distances
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 1;

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut ways_to_win = 0;

        for i in 1..=time {
            let my_distance = i * (time - i);

            if my_distance > distance {
                ways_to_win += 1;
            }
        }

        result *= ways_to_win;
    }

    result
}

#[test]
fn test_part1() {
    assert_eq!(part1("Time:      7  15   30\nDistance:  9  40  200"), 288);
}
