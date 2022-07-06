use std::collections::HashSet;

const FILE: &str = "inputs/day4.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // To ensure security, a valid passphrase must contain no duplicate words.
    // The system's full passphrase list is available as your puzzle input.
    // How many passphrases are valid?
    let part1 = input
        .lines()
        .filter(|&passphrase| {
            let mut words: HashSet<&str> = HashSet::default();
            passphrase.split_whitespace().all(|word| words.insert(word))
        })
        .count();

    println!("Part 1: {}", part1);

    // For added security, yet another system policy has been put in place.
    // Now, a valid passphrase must contain no two words that are anagrams of
    // each other.
    // Under this new system policy, how many passphrases are valid?
    let part2 = input
        .lines()
        .filter(|&passphrase| {
            let mut words: HashSet<Vec<char>> = HashSet::default();
            passphrase.split_whitespace().all(|word| {
                let mut word: Vec<char> = word.chars().collect();
                word.sort_unstable();
                words.insert(word)
            })
        })
        .count();

    println!("Part 2: {}", part2);

    Ok(())
}
