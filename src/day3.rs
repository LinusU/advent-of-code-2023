#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Symbol(char),
    Digit(u8),
}

impl Cell {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Cell::Empty),
            '0' => Ok(Cell::Digit(0)),
            '1' => Ok(Cell::Digit(1)),
            '2' => Ok(Cell::Digit(2)),
            '3' => Ok(Cell::Digit(3)),
            '4' => Ok(Cell::Digit(4)),
            '5' => Ok(Cell::Digit(5)),
            '6' => Ok(Cell::Digit(6)),
            '7' => Ok(Cell::Digit(7)),
            '8' => Ok(Cell::Digit(8)),
            '9' => Ok(Cell::Digit(9)),
            '-' => Ok(Cell::Symbol('-')),
            '@' => Ok(Cell::Symbol('@')),
            '*' => Ok(Cell::Symbol('*')),
            '/' => Ok(Cell::Symbol('/')),
            '&' => Ok(Cell::Symbol('&')),
            '#' => Ok(Cell::Symbol('#')),
            '%' => Ok(Cell::Symbol('%')),
            '+' => Ok(Cell::Symbol('+')),
            '=' => Ok(Cell::Symbol('=')),
            '$' => Ok(Cell::Symbol('$')),
            _ => Err(format!("Invalid cell: {}", c)),
        }
    }
}

trait EngineSchematic {
    fn cell_at(&self, x: isize, y: isize) -> Option<&Cell>;
    fn has_symbol_adjecent(&self, x: isize, y: isize) -> bool;
    fn height(&self) -> isize;
    fn read_part_number(&self, x: isize, y: isize) -> u64;
    fn width(&self) -> isize;
}

impl EngineSchematic for Vec<Vec<Cell>> {
    fn cell_at(&self, x: isize, y: isize) -> Option<&Cell> {
        if x < 0 || y < 0 {
            return None;
        }

        self.get(y as usize).and_then(|line| line.get(x as usize))
    }

    fn has_symbol_adjecent(&self, x: isize, y: isize) -> bool {
        let mut has_symbol_adjecent = false;

        for y in (y - 1)..=(y + 1) {
            for x in (x - 1)..=(x + 1) {
                if x == 0 && y == 0 {
                    continue;
                }

                if let Some(Cell::Symbol(_)) = self.cell_at(x, y) {
                    has_symbol_adjecent = true;
                }
            }
        }

        has_symbol_adjecent
    }

    fn height(&self) -> isize {
        self.len() as isize
    }

    fn read_part_number(&self, x: isize, y: isize) -> u64 {
        let mut start = x;

        while start > 0 {
            if let Some(Cell::Digit(_)) = self.cell_at(start - 1, y) {
                start -= 1;
                continue;
            } else {
                break;
            }
        }

        let mut n = 0;

        for x in start..self.width() {
            if let Some(Cell::Digit(d)) = self.cell_at(x, y) {
                n = (n * 10) + (*d as u64);
            } else {
                break;
            }
        }

        n
    }

    fn width(&self) -> isize {
        self.get(0).map(|line| line.len() as isize).unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ParserState {
    Empty,
    PartNumber(bool, u64),
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let schematic = input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| Cell::from_char(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..schematic.height() {
        let mut state = ParserState::Empty;

        for x in 0..=schematic.width() {
            state = match (state, schematic.cell_at(x, y).unwrap_or(&Cell::Empty)) {
                (ParserState::Empty, Cell::Digit(n)) => {
                    ParserState::PartNumber(schematic.has_symbol_adjecent(x, y), *n as u64)
                }
                (ParserState::PartNumber(has_symbol_adjecent, n), Cell::Digit(m)) => {
                    ParserState::PartNumber(
                        has_symbol_adjecent || schematic.has_symbol_adjecent(x, y),
                        (n * 10) + (*m as u64),
                    )
                }
                (ParserState::PartNumber(has_symbol_adjecent, n), _) => {
                    if has_symbol_adjecent {
                        sum += n;
                    }

                    ParserState::Empty
                }
                (ParserState::Empty, _) => ParserState::Empty,
            }
        }

        if let ParserState::PartNumber(true, n) = state {
            sum += n;
        }
    }

    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let schematic = input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| Cell::from_char(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..schematic.height() {
        for x in 0..schematic.width() {
            if let Some(Cell::Symbol('*')) = schematic.cell_at(x, y) {
                let mut part_numbers = Vec::<u64>::new();

                if let Some(Cell::Digit(_)) = schematic.cell_at(x - 1, y - 1) {
                    part_numbers.push(schematic.read_part_number(x - 1, y - 1));

                    if let Some(Cell::Empty) = schematic.cell_at(x, y - 1) {
                        if let Some(Cell::Digit(_)) = schematic.cell_at(x + 1, y - 1) {
                            part_numbers.push(schematic.read_part_number(x + 1, y - 1));
                        }
                    }
                } else if let Some(Cell::Digit(_)) = schematic.cell_at(x, y - 1) {
                    part_numbers.push(schematic.read_part_number(x, y - 1));
                } else if let Some(Cell::Digit(_)) = schematic.cell_at(x + 1, y - 1) {
                    part_numbers.push(schematic.read_part_number(x + 1, y - 1));
                }

                if let Some(Cell::Digit(_)) = schematic.cell_at(x - 1, y) {
                    part_numbers.push(schematic.read_part_number(x - 1, y));
                }

                if let Some(Cell::Digit(_)) = schematic.cell_at(x + 1, y) {
                    part_numbers.push(schematic.read_part_number(x + 1, y));
                }

                if let Some(Cell::Digit(_)) = schematic.cell_at(x - 1, y + 1) {
                    part_numbers.push(schematic.read_part_number(x - 1, y + 1));

                    if let Some(Cell::Empty) = schematic.cell_at(x, y + 1) {
                        if let Some(Cell::Digit(_)) = schematic.cell_at(x + 1, y + 1) {
                            part_numbers.push(schematic.read_part_number(x + 1, y + 1));
                        }
                    }
                } else if let Some(Cell::Digit(_)) = schematic.cell_at(x, y + 1) {
                    part_numbers.push(schematic.read_part_number(x, y + 1));
                } else if let Some(Cell::Digit(_)) = schematic.cell_at(x + 1, y + 1) {
                    part_numbers.push(schematic.read_part_number(x + 1, y + 1));
                }

                if part_numbers.len() == 2 {
                    sum += part_numbers[0] * part_numbers[1];
                }
            }
        }
    }

    sum
}

#[test]
fn test_part1() {
    assert_eq!(part1("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 4361);
}

#[test]
fn test_part2() {
    assert_eq!(part2("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 467835);
}
