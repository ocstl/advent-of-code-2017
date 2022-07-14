use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

const FILE: &str = "inputs/day16.txt";

const NBR_PROGRAMS: usize = 16;
type Program = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(Program, Program),
}

impl FromStr for DanceMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, rest) = s.trim().split_at(1);
        Ok(match op {
            "s" => DanceMove::Spin(rest.parse().unwrap()),
            "x" => {
                let (a, b) = rest.split_once('/').unwrap();
                DanceMove::Exchange(a.parse().unwrap(), b.parse().unwrap())
            }
            "p" => {
                let (a, b) = rest.split_once('/').unwrap();
                DanceMove::Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
            }
            _ => return Err(s.to_string()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProgramLine([Program; NBR_PROGRAMS]);

impl ProgramLine {
    pub fn dance(mut self, dance_move: DanceMove) -> Self {
        match dance_move {
            DanceMove::Spin(size) => self.0.rotate_right(size),
            DanceMove::Exchange(a, b) => self.0.swap(a, b),
            DanceMove::Partner(a, b) => {
                for program in self.0.iter_mut() {
                    if *program == a {
                        *program = b;
                    } else if *program == b {
                        *program = a;
                    }
                }
            }
        }

        self
    }
}

impl Default for ProgramLine {
    fn default() -> Self {
        ProgramLine([
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ])
    }
}

impl Display for ProgramLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let dance_moves: Vec<DanceMove> = input.split(',').map(|d| d.parse().unwrap()).collect();

    // You watch the dance for a while and record their dance moves (your
    // puzzle input). In what order are the programs standing after their dance?
    let part1 = dance_moves
        .iter()
        .fold(ProgramLine::default(), |acc, &d| acc.dance(d));
    println!("Part 1: {}", part1);

    // In what order are the programs standing after their billion dances?
    // One billion is, sadly, not brute-forceable, though that is the point.
    // We'll iterate until we find a cycle, then find which of the previously
    // seen states is the right one.
    let mut states: HashMap<ProgramLine, usize> = HashMap::new();
    let mut iter = std::iter::successors(Some(ProgramLine::default()), |line| {
        Some(dance_moves.iter().fold(*line, |acc, &d| acc.dance(d)))
    })
    .enumerate();

    let previous = iter
        .find_map(|(idx, line)| states.insert(line, idx))
        .unwrap();
    let current = states.len();
    let period = current - previous;

    let target = previous + (1_000_000_000 % period);
    let part2 = states.iter().find(|(_, step)| **step == target).unwrap().0;
    println!("Part 2: {}", part2);

    Ok(())
}
