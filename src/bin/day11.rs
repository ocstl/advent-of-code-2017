use std::str::FromStr;

const FILE: &str = "inputs/day11.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "se" => Direction::SouthEast,
            "s" => Direction::South,
            "sw" => Direction::SouthWest,
            "nw" => Direction::NorthWest,
            _ => unreachable!("Invalid direction: {}", s),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct HexPosition {
    q: i32,
    r: i32,
    s: i32,
}

impl HexPosition {
    fn new(q: i32, r: i32, s: i32) -> Self {
        HexPosition { q, r, s }
    }

    fn distance_to_origin(self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }
}

impl std::ops::Add<Direction> for HexPosition {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => HexPosition::new(self.q, self.r - 1, self.s + 1),
            Direction::NorthEast => HexPosition::new(self.q + 1, self.r - 1, self.s),
            Direction::SouthEast => HexPosition::new(self.q + 1, self.r, self.s - 1),
            Direction::South => HexPosition::new(self.q, self.r + 1, self.s - 1),
            Direction::SouthWest => HexPosition::new(self.q - 1, self.r + 1, self.s),
            Direction::NorthWest => HexPosition::new(self.q - 1, self.r, self.s + 1),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let directions: Vec<Direction> = input
        .trim()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect();

    // You have the path the child process took. Starting where he started, you
    // need to determine the fewest number of steps required to reach him.
    let final_position = directions
        .iter()
        .fold(HexPosition::default(), |current, &d| current + d);
    let part1 = final_position.distance_to_origin();

    println!("Part 1: {}", part1);

    // How many steps away is the furthest he ever got from his starting
    // position?
    let part2 = directions
        .iter()
        .scan(HexPosition::default(), |current, d| {
            *current = *current + *d;
            Some(current.distance_to_origin())
        })
        .max()
        .unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}
