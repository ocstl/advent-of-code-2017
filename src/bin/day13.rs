const FILE: &str = "inputs/day13.txt";

#[derive(Debug, Default, Clone, Copy)]
struct Scanner {
    depth: u32,
    range: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    let scanners: Vec<Scanner> = input
        .lines()
        .map(|line| {
            let (left, right) = line.trim().split_once(": ").unwrap();
            let depth = left.parse().unwrap();
            let range = right.parse().unwrap();
            Scanner { depth, range }
        })
        .collect();

    // Given the details of the firewall you've recorded, if you leave
    // immediately, what is the severity of your whole trip?
    // If a scanner has range 3, it will take 4 picoseconds for it to come back
    // to the top row: 2 * (range - 1). Coupled with its depth, we can easily
    // find out which of them will catch us.
    let part1: u32 = scanners
        .iter()
        .filter_map(|scanner| {
            if scanner.depth % (2 * (scanner.range - 1)) == 0 {
                Some(scanner.depth * scanner.range)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", part1);

    // What is the fewest number of picoseconds that you need to delay the
    // packet to pass through the firewall without being caught?
    // Brute force. This reminds me of the Chinese Remainder Theorem, but I am
    // uncertain if it is applicable and/or usable. Brute force works well
    // enough though.
    let part2: u32 = (0..)
        .find(|delay| {
            scanners
                .iter()
                .all(|scanner| (scanner.depth + delay) % (2 * (scanner.range - 1)) != 0)
        })
        .unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}
