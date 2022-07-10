const FILE: &str = "inputs/day9.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    let mut count = 0;
    let mut garbage = 0;
    let mut depth = 0;

    let mut tokens = input.trim().chars();

    while let Some(token) = tokens.next() {
        match token {
            // Open a new group (thus increasing the current score).
            '{' => {
                depth += 1;
            }
            // Update the total (and current) score when we close a group.
            '}' => {
                count += depth;
                depth -= 1;
            }
            // Keep throwing away the garbage until we can close it.
            // Part 2: Add a counter for the number of elements we had to
            // throw away.
            '<' => {
                while let Some(token) = tokens.next() {
                    match token {
                        '!' => {
                            tokens.next();
                        }
                        '>' => break,
                        _ => {
                            garbage += 1;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // What is the total score for all groups in your input?
    println!("Part 1: {}", count);

    // How many non-canceled characters are within the garbage in your puzzle
    // input?
    println!("Part 2: {}", garbage);

    Ok(())
}
