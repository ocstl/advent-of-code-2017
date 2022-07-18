use advent_of_code_2017::intcode::{Computer, Instruction, Value};
use std::time::Duration;

const FILE: &str = "inputs/day18.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    // Once both of your programs have terminated (regardless of what caused
    // them to do so), how many times did program 1 send a value?
    // We'll act as the go-between for the sender-receiver channel from 1 to 0.
    let (s0, r1) = std::sync::mpsc::channel::<Value>();
    let (s1, r_from1) = std::sync::mpsc::channel::<Value>();
    let (s_from1, r0) = std::sync::mpsc::channel::<Value>();

    let instructions_0 = instructions.clone();
    std::thread::spawn(move || Computer::new(instructions_0, 0, s0, r0).run());
    std::thread::spawn(move || Computer::new(instructions, 1, s1, r1).run());

    let mut part2 = 0;

    while let Ok(value) = r_from1.recv_timeout(Duration::from_millis(100)) {
        let _ = s_from1.send(value);
        part2 += 1;
    }

    println!("Part 2: {}", part2);

    Ok(())
}
