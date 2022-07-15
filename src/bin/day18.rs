use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day18.txt";

type Register = char;
type Value = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parameter {
    Value(Value),
    Register(Register),
}

impl FromStr for Parameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().parse::<Value>().map_or_else(
            |_| {
                s.chars()
                    .next()
                    .map(Parameter::Register)
                    .ok_or_else(|| s.to_string())
            },
            |v| Ok(Parameter::Value(v)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Sound(Parameter),
    Set(Register, Parameter),
    Add(Register, Parameter),
    Multiply(Register, Parameter),
    Modulo(Register, Parameter),
    Recover(Parameter),
    JumpGreaterThanZero(Register, Parameter),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let op = iter.next().ok_or("Missing operation.")?;
        let first = iter.next().ok_or("Missing first parameter.")?;
        let second = iter.next();

        match op {
            "snd" => first.parse().map(Instruction::Sound),
            "set" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                second
                    .ok_or_else(|| s.to_string())?
                    .parse()
                    .map(|p| Instruction::Set(register, p))
            }
            "add" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                second
                    .ok_or_else(|| s.to_string())?
                    .parse()
                    .map(|p| Instruction::Add(register, p))
            }
            "mul" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                second
                    .ok_or_else(|| s.to_string())?
                    .parse()
                    .map(|p| Instruction::Multiply(register, p))
            }
            "mod" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                second
                    .ok_or_else(|| s.to_string())?
                    .parse()
                    .map(|p| Instruction::Modulo(register, p))
            }
            "rcv" => first.parse().map(Instruction::Recover),
            "jgz" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                second
                    .ok_or_else(|| s.to_string())?
                    .parse()
                    .map(|p| Instruction::JumpGreaterThanZero(register, p))
            }
            _ => Err(format!("Unknown operation: {}", op)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Computer {
    registers: HashMap<Register, Value>,
    instructions: Vec<Instruction>,
    last_played: Value,
    instruction_pointer: usize,
}

impl Computer {
    pub fn with_instructions(instructions: Vec<Instruction>) -> Self {
        Computer {
            registers: HashMap::default(),
            instructions,
            last_played: 0,
            instruction_pointer: 0,
        }
    }

    fn get_value(&self, parameter: Parameter) -> Value {
        match parameter {
            Parameter::Value(v) => v,
            Parameter::Register(r) => self.registers.get(&r).copied().unwrap_or_default(),
        }
    }

    fn step(&mut self) -> Option<Value> {
        match self.instructions[self.instruction_pointer] {
            Instruction::Sound(p) => {
                self.last_played = self.get_value(p);
                self.instruction_pointer += 1;
                None
            }
            Instruction::Set(r, p) => {
                self.registers.insert(r, self.get_value(p));
                self.instruction_pointer += 1;
                None
            }
            Instruction::Add(r, p) => {
                *self.registers.entry(r).or_default() += self.get_value(p);
                self.instruction_pointer += 1;
                None
            }
            Instruction::Multiply(r, p) => {
                *self.registers.entry(r).or_default() *= self.get_value(p);
                self.instruction_pointer += 1;
                None
            }
            Instruction::Modulo(r, p) => {
                *self.registers.entry(r).or_default() %= self.get_value(p);
                self.instruction_pointer += 1;
                None
            }
            Instruction::Recover(p) => {
                self.instruction_pointer += 1;
                if self.get_value(p) != 0 {
                    Some(self.last_played)
                } else {
                    None
                }
            }
            Instruction::JumpGreaterThanZero(r, p) => {
                if *self.registers.entry(r).or_default() > 0 {
                    self.instruction_pointer =
                        (self.instruction_pointer as Value + self.get_value(p)) as usize;
                } else {
                    // This is necessary, otherwise we will never get out.
                    self.instruction_pointer += 1;
                }
                None
            }
        }
    }
}

impl Iterator for Computer {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.step();
        while current.is_none() {
            current = self.step();
        }

        current
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let instructions = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    // What is the value of the recovered frequency (the value of the most
    // recently played sound) the first time a rcv instruction is executed with
    // a non-zero value?
    let part1 = Computer::with_instructions(instructions).next().unwrap();
    println!("Part 1: {}", part1);

    Ok(())
}
