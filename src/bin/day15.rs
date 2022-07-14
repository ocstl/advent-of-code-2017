const FILE: &str = "inputs/day15.txt";
const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const FILTER_A: u64 = 4;
const FILTER_B: u64 = 8;

#[derive(Debug, Default, Clone, Copy)]
struct Generator {
    factor: u64,
    state: u64,
}

impl Generator {
    const MODULUS: u64 = 2147483647;

    pub fn new(factor: u64, start: u64) -> Self {
        Generator {
            factor,
            state: start,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.state *= self.factor;
        self.state %= Self::MODULUS;
        Some(self.state)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut starts = input.lines().map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    });

    let generator_a = Generator::new(FACTOR_A, starts.next().unwrap());
    let generator_b = Generator::new(FACTOR_B, starts.next().unwrap());

    // After 40 million pairs, what is the judge's final count?
    let part1 = generator_a
        .zip(generator_b)
        .take(40_000_000)
        .filter(|(a, b)| a & 0xffff == b & 0xffff)
        .count();
    println!("Part 1: {}", part1);

    // After 5 million pairs, but using this new generator logic, what is the
    // judge's final count?
    let part2 = generator_a
        .filter(|a| a % FILTER_A == 0)
        .zip(generator_b.filter(|b| b % FILTER_B == 0))
        .take(5_000_000)
        .filter(|(a, b)| a & 0xffff == b & 0xffff)
        .count();
    println!("Part 2: {}", part2);

    Ok(())
}
