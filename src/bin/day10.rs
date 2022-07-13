use advent_of_code_2017::knot_hash::KnotHash;

const FILE: &str = "inputs/day10.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // Once this process is complete, what is the result of multiplying the
    // first two numbers in the list?
    let mut knot = KnotHash::default();
    let lengths = input
        .split(',')
        .map(|length| length.trim().parse().unwrap());
    knot.round(lengths);
    let output = knot.get_state();
    let part1 = u32::from(output[0]) * u32::from(output[1]);

    println!("Part 1: {}", part1);

    // Treating your puzzle input as a string of ASCII characters, what is the
    // Knot Hash of your puzzle input? Ignore any leading or trailing
    // whitespace you might encounter.
    let mut knot = KnotHash::default();
    knot.hash(input.trim().bytes());
    let part2 = knot.to_string();

    println!("Part 2: {}", part2);

    Ok(())
}
