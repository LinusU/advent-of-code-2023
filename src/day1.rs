use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

            (first as u64 - '0' as u64) * 10 + (last as u64 - '0' as u64)
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
}
