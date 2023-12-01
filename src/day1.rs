use aoc_runner_derive::aoc;
use bstr::ByteSlice;

const DIGIT_0: u8 = b'0';
const DIGIT_9: u8 = b'9';

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

fn digit(input: &[u8]) -> Option<u8> {
    if input[0] >= DIGIT_0 && input[0] <= DIGIT_9 {
        Some(input[0] - DIGIT_0)
    } else if input.starts_with_str("one") {
        Some(1)
    } else if input.starts_with_str("two") {
        Some(2)
    } else if input.starts_with_str("three") {
        Some(3)
    } else if input.starts_with_str("four") {
        Some(4)
    } else if input.starts_with_str("five") {
        Some(5)
    } else if input.starts_with_str("six") {
        Some(6)
    } else if input.starts_with_str("seven") {
        Some(7)
    } else if input.starts_with_str("eight") {
        Some(8)
    } else if input.starts_with_str("nine") {
        Some(9)
    } else {
        None
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &[u8]) -> u64 {
    input
        .split_str("\n")
        .map(|line| {
            let len = line.len();
            let mut pos = 0;

            let first = loop {
                if let Some(digit) = digit(&line[pos..]) {
                    break digit;
                }

                pos += 1;
            };

            pos = len - 1;

            let last = loop {
                if let Some(digit) = digit(&line[pos..]) {
                    break digit;
                }

                pos -= 1;
            };

            (first as u64) * 10 + (last as u64)
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
}

#[test]
fn test_part2() {
    assert_eq!(part2(b"two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
}
