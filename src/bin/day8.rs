use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day8.txt";

type Register = String;
type Registers = HashMap<Register, i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Equal(i32),
    NotEqual(i32),
    LessThan(i32),
    LessOrEqual(i32),
    GreaterThan(i32),
    GreaterOrEqual(i32),
}

#[derive(Debug, Clone)]
struct Instruction {
    register: Register,
    modification: i32,
    cond_register: Register,
    condition: Condition,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, condition) = s.split_once(" if ").unwrap();

        // Instruction part.
        let mut iter = instruction.split_whitespace();
        let register = iter.next().unwrap().to_string();
        let op = if iter.next().unwrap() == "inc" { 1 } else { -1 };
        let modification = iter.next().unwrap().parse::<i32>().unwrap() * op;

        // Condition part.
        let mut iter = condition.split_whitespace();
        let cond_register = iter.next().unwrap().to_string();
        let op = iter.next().unwrap();
        let target: i32 = iter.next().unwrap().parse().unwrap();
        let condition = match op {
            "==" => Condition::Equal(target),
            "!=" => Condition::NotEqual(target),
            "<" => Condition::LessThan(target),
            "<=" => Condition::LessOrEqual(target),
            ">" => Condition::GreaterThan(target),
            ">=" => Condition::GreaterOrEqual(target),
            _ => unreachable!("Invalid comparison operator."),
        };

        Ok(Instruction {
            register,
            modification,
            cond_register,
            condition,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // What is the largest value in any register after completing the
    // instructions in your puzzle input?
    // Part 2: To be safe, the CPU also needs to know the highest value held in
    // any register during this process so that it can decide how much memory
    // to allocate to these operations.
    let mut registers = Registers::new();
    let mut part2 = 0;
    for line in input.lines() {
        let Instruction {
            register,
            modification,
            cond_register,
            condition,
        } = line.parse().unwrap();

        let value = registers.get(&cond_register).copied().unwrap_or_default();
        if match condition {
            Condition::Equal(target) => value.eq(&target),
            Condition::NotEqual(target) => value.ne(&target),
            Condition::LessThan(target) => value.lt(&target),
            Condition::LessOrEqual(target) => value.le(&target),
            Condition::GreaterThan(target) => value.gt(&target),
            Condition::GreaterOrEqual(target) => value.ge(&target),
        } {
            // Keep track of the maximum for part 2.
            let value = registers.entry(register).or_default();
            *value += modification;
            part2 = part2.max(*value);
        }
    }

    let part1 = registers.values().max().unwrap();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
