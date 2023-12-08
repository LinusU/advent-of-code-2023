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

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();

    let time = time.strip_prefix("Time:").unwrap();
    let distance = distance.strip_prefix("Distance:").unwrap();

    let time = time.replace(' ', "").parse::<u64>().unwrap();
    let distance = distance.replace(' ', "").parse::<u64>().unwrap();

    // Find roots of the quadratic equation:
    // x^2 - (time * x) + distance = 0
    // a = 1
    // b = -time
    // c = distance
    let discriminant = (time * time) - (4 * distance);

    let middle = (time as f64) / 2.0;
    let half_sqrt_discriminant = (discriminant as f64).sqrt() / 2.0;

    let first_winning = (middle - half_sqrt_discriminant).ceil() as u64;
    let last_winning = (middle + half_sqrt_discriminant).floor() as u64;

    (first_winning..=last_winning).count() as u64
}

#[test]
fn test_part1() {
    assert_eq!(part1("Time:      7  15   30\nDistance:  9  40  200"), 288);
}

#[test]
fn test_part2() {
    assert_eq!(part2("Time:      7  15   30\nDistance:  9  40  200"), 71503);
}
