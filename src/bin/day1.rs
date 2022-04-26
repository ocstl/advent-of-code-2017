const FILE: &str = "inputs/day1.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let digits = input
        .trim()
        .chars()
        .map(|b| b.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .unwrap_or_default();

    // What is the solution to your captcha?
    let part1: u32 = digits
        .iter()
        .zip(digits.iter().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();
    println!("Part 1: {}", part1);

    // What is the solution to your new captcha?
    let part2: u32 = digits
        .iter()
        .zip(digits.iter().cycle().skip(digits.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}
