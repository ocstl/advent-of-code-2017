use std::collections::HashSet;

const FILE: &str = "inputs/day12.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let pipes: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let (_, others) = line.trim().split_once(" <-> ").unwrap();
            others
                .split(',')
                .map(|other| other.trim().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // How many programs are in the group that contains program ID 0?
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();
    to_visit.push(0);

    while let Some(program) = to_visit.pop() {
        visited.insert(program);
        to_visit.extend(pipes[program].iter().filter(|p| !visited.contains(p)));
    }

    let part1 = visited.len();
    println!("Part 1: {}", part1);

    // How many groups are there in total?
    // We can start from where part 1 left us, with the 0 group.
    let mut count = 1;

    for start in 0..pipes.len() {
        if !visited.contains(&start) {
            count += 1;
            let mut to_visit = Vec::new();
            to_visit.push(start);

            while let Some(program) = to_visit.pop() {
                visited.insert(program);
                to_visit.extend(pipes[program].iter().filter(|p| !visited.contains(p)));
            }
        }
    }

    println!("Part 2: {}", count);

    Ok(())
}
