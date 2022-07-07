const FILE: &str = "inputs/day5.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // How many steps does it take to reach the exit?
    let mut instructions: Vec<isize> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut idx: isize = 0;
    let mut count: u32 = 0;
    while let Some(instruction) = instructions.get_mut(idx as usize) {
        count += 1;
        idx += *instruction;
        *instruction += 1;
    }

    println!("Part 1: {}", count);

    // How many steps does it now take to reach the exit?
    let mut instructions: Vec<isize> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut idx: isize = 0;
    let mut count: u32 = 0;
    while let Some(instruction) = instructions.get_mut(idx as usize) {
        count += 1;
        idx += *instruction;
        if *instruction >= 3 {
            *instruction -= 1;
        } else {
            *instruction += 1;
        }
    }

    println!("Part 2: {}", count);
    Ok(())
}
