const FILE: &str = "inputs/day2.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let spreadsheet = input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()?;

    // What is the checksum for the spreadsheet in your puzzle input?
    let part1: u32 = spreadsheet
        .iter()
        .map(|row| row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0))
        .sum();
    println!("Part 1: {}", part1);

    // What is the sum of each row's result in your puzzle input?
    let part2: u32 = spreadsheet
        .iter()
        .map(|row| {
            let mut row = row.clone();
            row.sort_unstable();

            // A larger number has a better chance to be divisible than a
            // smaller one. Also, this will catch cases where the same number
            // appears twice (not clear if it's possible or not though).
            while let Some(large) = row.pop() {
                if let Some(divisor) = row.iter().find(|&&d| large % d == 0) {
                    return large / divisor;
                }
            }

            0
        })
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}
