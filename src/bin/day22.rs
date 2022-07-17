use advent_of_code_2017::position::{Direction, Position};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const FILE: &str = "inputs/day22.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    #[default]
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '.' => Node::Clean,
            '#' => Node::Infected,
            'W' => Node::Weakened,
            'F' => Node::Flagged,
            _ => unreachable!("Invalid character."),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cluster(HashMap<Position, Node>);

impl FromStr for Cluster {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = (s.lines().count() - 1) as isize;
        Ok(Self(
            s.lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices().map(move |(col, c)| {
                        (
                            Position::new(col as isize, rows - (row as isize)),
                            Node::from(c),
                        )
                    })
                })
                .collect(),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirusCarrier {
    position: Position,
    direction: Direction,
}

impl VirusCarrier {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    pub fn reverse(&mut self) {
        self.direction = self.direction.reverse();
    }

    pub fn forward(&mut self) {
        self.position = self.position + self.direction;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut cluster = Cluster::from_str(&input)?;
    let initial = VirusCarrier::new(
        Position::new(
            (input.lines().count() / 2) as isize,
            (input.lines().next().unwrap().len() / 2) as isize,
        ),
        Direction::Up,
    );

    // Given your actual map, after 10000 bursts of activity, how many bursts
    // cause a node to become infected? (Do not count nodes that begin
    // infected.)
    let mut infections = 0;
    let mut carrier = initial;
    let mut part1: HashSet<Position> = cluster
        .0
        .iter()
        .filter_map(|(&p, &n)| if n == Node::Infected { Some(p) } else { None })
        .collect();
    for _ in 0..10000 {
        if part1.remove(&carrier.position) {
            carrier.turn_right();
        } else {
            carrier.turn_left();
            part1.insert(carrier.position);
            infections += 1;
        }

        carrier.forward();
    }

    println!("Part 1: {}", infections);

    // Given your actual map, after 10000000 bursts of activity, how many
    // bursts cause a node to become infected? (Do not count nodes that begin
    // infected.)
    let mut infections = 0;
    let mut carrier = initial;
    for _ in 0..10000000 {
        let e = cluster.0.entry(carrier.position).or_default();
        match e {
            Node::Clean => {
                carrier.turn_left();
                *e = Node::Weakened;
            }
            Node::Weakened => {
                *e = Node::Infected;
                infections += 1;
            }
            Node::Infected => {
                carrier.turn_right();
                *e = Node::Flagged;
            }
            Node::Flagged => {
                carrier.reverse();
                *e = Node::Clean;
            }
        }

        carrier.forward();
    }

    println!("Part 2: {}", infections);

    Ok(())
}
