use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::time::Duration;

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
    Send(Parameter),
    Set(Register, Parameter),
    Add(Register, Parameter),
    Multiply(Register, Parameter),
    Modulo(Register, Parameter),
    Receive(Register),
    JumpGreaterThanZero(Parameter, Parameter),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let op = iter.next().ok_or("Missing operation.")?;
        let first = iter.next().ok_or("Missing first parameter.")?;
        let second = iter.next();

        match op {
            "snd" => first.parse().map(Instruction::Send),
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
            "rcv" => {
                let register = first.chars().next().expect(
                    "There should be at least one character, since we used `split_whitespace`.",
                );
                Ok(Instruction::Receive(register))
            }
            "jgz" => {
                let first = first.parse()?;
                let second = second
                    .ok_or_else(|| s.to_string())
                    .and_then(|second| second.parse())?;
                Ok(Instruction::JumpGreaterThanZero(first, second))
            }
            _ => Err(format!("Unknown operation: {}", op)),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    registers: HashMap<Register, Value>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    sender: Sender<Value>,
    receiver: Receiver<Value>,
}

impl Computer {
    pub fn new(
        instructions: Vec<Instruction>,
        p: Value,
        sender: Sender<Value>,
        receiver: Receiver<Value>,
    ) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', p);

        Computer {
            registers,
            instructions,
            instruction_pointer: 0,
            sender,
            receiver,
        }
    }

    fn get_value(&self, parameter: Parameter) -> Value {
        match parameter {
            Parameter::Value(v) => v,
            Parameter::Register(r) => self.registers.get(&r).copied().unwrap_or_default(),
        }
    }

    fn step(&mut self) -> Result<(), RecvError> {
        match self.instructions[self.instruction_pointer] {
            Instruction::Send(p) => {
                let _ = self.sender.send(self.get_value(p));
                self.instruction_pointer += 1;
            }
            Instruction::Set(r, p) => {
                self.registers.insert(r, self.get_value(p));
                self.instruction_pointer += 1;
            }
            Instruction::Add(r, p) => {
                *self.registers.entry(r).or_default() += self.get_value(p);
                self.instruction_pointer += 1;
            }
            Instruction::Multiply(r, p) => {
                *self.registers.entry(r).or_default() *= self.get_value(p);
                self.instruction_pointer += 1;
            }
            Instruction::Modulo(r, p) => {
                *self.registers.entry(r).or_default() %= self.get_value(p);
                self.instruction_pointer += 1;
            }
            Instruction::Receive(p) => {
                self.registers.insert(p, self.receiver.recv()?);
                self.instruction_pointer += 1;
            }
            Instruction::JumpGreaterThanZero(r, p) => {
                if self.get_value(r) > 0 {
                    self.instruction_pointer =
                        (self.instruction_pointer as Value + self.get_value(p)) as usize;
                } else {
                    // This is necessary, otherwise we will never get out.
                    self.instruction_pointer += 1;
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) {
        while self.step().is_ok() {}
    }
}

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
