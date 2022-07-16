use advent_of_code_2017::position::Direction;
use std::str::FromStr;

const FILE: &str = "inputs/day19.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Path,
    Letter(char),
    Turn,
    Empty,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '|' | '-' => Square::Path,
            '+' => Square::Turn,
            ' ' => Square::Empty,
            _ => Square::Letter(c),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Diagram(Vec<Vec<Square>>);

impl Diagram {
    pub fn start(&self) -> Position {
        self.0
            .first()
            .unwrap()
            .iter()
            .position(|&p| p == Square::Path)
            .map(|x| Position::new(x, 0))
            .unwrap()
    }
}

impl FromStr for Diagram {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Diagram(
            s.lines()
                .map(|line| line.chars().map(Square::from).collect())
                .collect(),
        ))
    }
}

impl std::ops::Index<Position> for Diagram {
    type Output = Square;

    fn index(&self, index: Position) -> &Self::Output {
        self.0
            .get(index.y)
            .and_then(|row| row.get(index.x))
            .unwrap_or(&Square::Empty)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let diagram: Diagram = input.parse().unwrap();
    let start = diagram.start();

    // What letters will it see (in the order it would see them) if it follows
    // the path?
    // Part 2: How many steps does the packet need to go?
    let mut position = start;
    let mut direction = Direction::Down;
    let mut steps = 0;
    let mut path = String::new();

    loop {
        match diagram[position] {
            Square::Path => (),
            Square::Turn => {
                if diagram[position + direction.turn_left()] != Square::Empty {
                    direction = direction.turn_left();
                } else if diagram[position + direction.turn_right()] != Square::Empty {
                    direction = direction.turn_right();
                }
            }
            Square::Letter(c) => {
                path.push(c);
            }
            Square::Empty => break,
        }

        position = position + direction;
        steps += 1;
    }

    println!("Part 1: {}", path);
    println!("Part 2: {}", steps);

    Ok(())
}
