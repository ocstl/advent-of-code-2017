use advent_of_code_2017::intcode::{Computer, Instruction};
use primal::is_prime;
use std::sync::mpsc::channel;

const FILE: &str = "inputs/day23.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    // If you run the program (your puzzle input), how many times is the mul
    // instruction invoked?
    let (s0, r0) = channel();
    let part1 = Computer::new(instructions, 0, s0, r0).debug_mode();
    println!("Part 1: {}", part1);

    // Reading the program:
    // * magic numbers: INIT, SPAN, STEP (SPAN is a multiple of step)
    // * iterate from INIT..=(INIT + SPAN) by STEP:
    //   if CURRENT is composite, increment register h
    // It is incredibly inefficient, since it iterates 2 nested loops (2 to
    // CURRENT) for every check (not even short-circuiting!).
    // We'll do it a different way instead.
    let part2 = (93 * 100 + 100_000..=93 * 100 + 100_000 + 17_000)
        .step_by(17)
        .filter(|&n| !is_prime(n))
        .count();
    println!("Part 2: {}", part2);

    Ok(())
}
