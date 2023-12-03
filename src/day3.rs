#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Symbol,
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
            '-' => Ok(Cell::Symbol),
            '@' => Ok(Cell::Symbol),
            '*' => Ok(Cell::Symbol),
            '/' => Ok(Cell::Symbol),
            '&' => Ok(Cell::Symbol),
            '#' => Ok(Cell::Symbol),
            '%' => Ok(Cell::Symbol),
            '+' => Ok(Cell::Symbol),
            '=' => Ok(Cell::Symbol),
            '$' => Ok(Cell::Symbol),
            _ => Err(format!("Invalid cell: {}", c)),
        }
    }
}

trait EngineSchematic {
    fn cell_at(&self, x: usize, y: usize) -> Option<&Cell>;
    fn has_symbol_adjecent(&self, x: usize, y: usize) -> bool;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl EngineSchematic for Vec<Vec<Cell>> {
    fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.get(y).and_then(|line| line.get(x))
    }

    fn has_symbol_adjecent(&self, x: usize, y: usize) -> bool {
        let mut has_symbol_adjecent = false;

        let start_x = if x == 0 { 0 } else { x - 1 };
        let start_y = if y == 0 { 0 } else { y - 1 };

        for y in start_y..=y + 1 {
            for x in start_x..=x + 1 {
                if x == 0 && y == 0 {
                    continue;
                }

                if let Some(Cell::Symbol) = self.cell_at(x, y) {
                    has_symbol_adjecent = true;
                }
            }
        }

        has_symbol_adjecent
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        self.get(0).map(|line| line.len()).unwrap_or(0)
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

#[test]
fn test_part1() {
    assert_eq!(part1("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 4361);
}
