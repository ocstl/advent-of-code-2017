use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

const FILE: &str = "inputs/day21.txt";
const INPUT: [[Pixel; 3]; 3] = [
    [Pixel::Off, Pixel::On, Pixel::Off],
    [Pixel::Off, Pixel::Off, Pixel::On],
    [Pixel::On, Pixel::On, Pixel::On],
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pixel {
    On,
    #[default]
    Off,
}

impl TryFrom<char> for Pixel {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Pixel::On),
            '.' => Ok(Pixel::Off),
            _ => Err(value),
        }
    }
}

impl From<Pixel> for char {
    fn from(p: Pixel) -> char {
        match p {
            Pixel::On => '#',
            Pixel::Off => '.',
        }
    }
}

#[derive(Debug, Clone)]
pub struct PixelGrid {
    grid: Vec<Vec<Pixel>>,
    rules: HashMap<Vec<Pixel>, Vec<Vec<Pixel>>>,
}

impl PixelGrid {
    pub fn enhance(&mut self) {
        // Oh, this is horrible.
        let (step, mut grid) = if self.grid.len() % 2 == 0 {
            (
                2,
                vec![vec![Pixel::default(); 3 * self.grid.len() / 2]; 3 * self.grid.len() / 2],
            )
        } else {
            (
                3,
                vec![vec![Pixel::default(); 4 * self.grid.len() / 3]; 4 * self.grid.len() / 3],
            )
        };

        for row in (0..self.grid.len()).step_by(step) {
            for col in (0..self.grid.len()).step_by(step) {
                let pattern: Vec<Pixel> = self.grid[row..row + step]
                    .iter()
                    .flat_map(|r| r[col..col + step].iter())
                    .copied()
                    .collect();

                let out_row = (row / step) * (step + 1);
                let out_col = (col / step) * (step + 1);
                let output = self.rules.get(&pattern).expect("Missing pattern.");
                for (idy, r) in output.iter().enumerate() {
                    for (idx, p) in r.iter().enumerate() {
                        grid[out_row + idy][out_col + idx] = *p;
                    }
                }
            }
        }

        self.grid = grid;
    }

    pub fn count_pixels(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|p| **p == Pixel::On)
            .count()
    }
}

impl FromStr for PixelGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = INPUT.into_iter().map(|row| row.to_vec()).collect();
        let mut rules = HashMap::new();

        // Rather than deal with matrices, we can treat patterns as flat arrays.
        // Generating all possibilities of rotations/flips to create all rules
        // is simpler than performing these when matching, especially since
        // there are only 16 2x2 patterns (2^4) and 512 3x3 patterns (2^9).
        for line in s.lines() {
            let (pattern, output) = line.split_once(" => ").expect("Bad formatting.");
            let mut pattern: Vec<Pixel> = pattern
                .chars()
                .flat_map(|c| Pixel::try_from(c).ok())
                .collect();
            let output: Vec<Vec<Pixel>> = output
                .split('/')
                .map(|row| row.chars().flat_map(|c| Pixel::try_from(c).ok()).collect())
                .collect();

            rules.insert(pattern.clone(), output.clone());
            // Rotate four times. The flipped version as well.
            match pattern.len() {
                4 => {
                    for _ in 0..4 {
                        let flip_ud = [pattern[2], pattern[3], pattern[0], pattern[1]].to_vec();
                        let flip_lr = [pattern[1], pattern[0], pattern[3], pattern[2]].to_vec();
                        rules.insert(flip_ud, output.clone());
                        rules.insert(flip_lr, output.clone());

                        pattern = [pattern[1], pattern[3], pattern[0], pattern[2]].to_vec();
                        rules.insert(pattern.clone(), output.clone());
                    }
                }
                9 => {
                    for _ in 0..4 {
                        let flip_ud = [
                            pattern[6], pattern[7], pattern[8], pattern[3], pattern[4], pattern[5],
                            pattern[0], pattern[1], pattern[2],
                        ]
                        .to_vec();
                        let flip_lr = [
                            pattern[2], pattern[1], pattern[0], pattern[5], pattern[4], pattern[3],
                            pattern[8], pattern[7], pattern[6],
                        ]
                        .to_vec();
                        rules.insert(flip_ud, output.clone());
                        rules.insert(flip_lr, output.clone());

                        pattern = [
                            pattern[2], pattern[5], pattern[8], pattern[1], pattern[4], pattern[7],
                            pattern[0], pattern[3], pattern[6],
                        ]
                        .to_vec();
                        rules.insert(pattern.clone(), output.clone());
                    }
                }
                _ => unreachable!("Wrong sized pattern."),
            }
        }

        Ok(Self { grid, rules })
    }
}

impl Display for PixelGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Vec<String> = self
            .grid
            .iter()
            .map(|row| row.iter().map(|&p| char::from(p)).collect())
            .collect();
        write!(f, "{}", s.join("\n"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut grid = PixelGrid::from_str(&input).unwrap();

    // How many pixels stay on after 5 iterations?
    for _ in 0..5 {
        grid.enhance();
    }
    println!("Part 1: {}", grid.count_pixels());

    // How many pixels stay on after 18 iterations?
    for _ in 5..18 {
        grid.enhance();
    }
    println!("Part 2: {}", grid.count_pixels());

    Ok(())
}
