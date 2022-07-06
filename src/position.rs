#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    pub fn neighbours(self) -> [Position; 8] {
        [
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x + 1, self.y + 1),
        ]
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
