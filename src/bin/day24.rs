const FILE: &str = "inputs/day24.txt";

type Component = (u32, u32);

fn strongest(port: u32, available: &[Component]) -> u32 {
    available
        .iter()
        .enumerate()
        .map(|(idx, &component)| match component {
            (x, y) if x == port => {
                let mut remaining = available.to_vec();
                remaining.swap_remove(idx);
                x + y + strongest(y, &remaining)
            }
            (x, y) if y == port => {
                let mut remaining = available.to_vec();
                remaining.swap_remove(idx);
                x + y + strongest(x, &remaining)
            }
            _ => 0,
        })
        .max()
        .unwrap()
}

fn longest(port: u32, available: &[Component]) -> (u32, u32) {
    // Using a tuple will allow to max over the length first, then over the
    // strength.
    available
        .iter()
        .enumerate()
        .map(|(idx, &component)| match component {
            (x, y) if x == port => {
                let mut remaining = available.to_vec();
                remaining.swap_remove(idx);
                let l = longest(y, &remaining);
                (1 + l.0, x + y + l.1)
            }
            (x, y) if y == port => {
                let mut remaining = available.to_vec();
                remaining.swap_remove(idx);
                let l = longest(x, &remaining);
                (1 + l.0, x + y + l.1)
            }
            _ => (0, 0),
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let components: Vec<Component> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('/').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    // What is the strength of the strongest bridge you can make with the
    // components you have available?
    let part1 = strongest(0, &components);
    println!("Part 1: {}", part1);

    // What is the strength of the longest bridge you can make? If you can
    // make multiple bridges of the longest length, pick the strongest one.
    let part2 = longest(0, &components);
    println!("Part 2: {}", part2.1);

    Ok(())
}
