use std::collections::HashMap;

const FILE: &str = "inputs/day6.txt";
const SIZE: usize = 16;

type Memory = [usize; SIZE];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // Given the initial block counts in your puzzle input, how many
    // redistribution cycles must be completed before a configuration is
    // produced that has been seen before?
    // Initially, a HashSet was used for Part 1, but we can combine both parts
    // into a HashMap instead, which will save some time.
    let mut memory = Memory::default();
    for (bank, value) in memory.iter_mut().zip(input.split_whitespace()) {
        *bank = value.parse().unwrap();
    }

    let debugger = std::iter::from_fn(move || {
        let max = *memory.iter().max().unwrap();
        let idx = memory
            .iter()
            .position(|&bank| bank == max)
            .unwrap_or_default();
        memory[idx] = 0;
        for idy in idx + 1..=idx + max {
            memory[idy % SIZE] += 1;
        }
        Some(memory)
    });

    // How many cycles are in the infinite loop that arises from the
    // configuration in your puzzle input?
    // Initially, a HashSet was used for Part 1, but we can combine both parts
    // using a HashMap instead, which will save some time. When inserting a pair
    // into a HashMap, if the key was already present, we get the former value
    // back; so, the first time we get a `Some`, we have hit a loop. Since this
    // is the first time we have seen a repetition, our HashMap will contain
    // N - 1 pairs, where N is the number of steps when we hit the loop.
    // To find the length of the cycle, we only have to find the difference
    // between the two number of steps (first and second hitting time).
    let mut seen = HashMap::new();
    let part2 = debugger
        .enumerate()
        .find_map(|(step, memory)| seen.insert(memory, step + 1))
        .unwrap();

    println!("Part 1: {}", seen.len() + 1);
    println!("Part 2: {}", seen.len() + 1 - part2);

    Ok(())
}
