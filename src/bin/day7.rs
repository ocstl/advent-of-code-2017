use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const FILE: &str = "inputs/day7.txt";

type Programs = HashMap<String, Program>;

#[derive(Debug, Default, Clone)]
struct Program {
    name: String,
    weight: i32,
    subprograms: Vec<String>,
}

impl Program {
    fn get_total_weight(&self, programs: &Programs) -> i32 {
        self.weight
            + self
                .subprograms
                .iter()
                .map(|sub| programs.get(sub).unwrap().get_total_weight(programs))
                .sum::<i32>()
    }

    fn unbalanced<'a>(&'a self, programs: &'a Programs) -> Option<(&'a Program, i32)> {
        let mut weights: HashMap<i32, Vec<&Program>> = HashMap::new();
        for sub in self.subprograms.iter() {
            let sub = programs.get(sub).unwrap();
            weights
                .entry(sub.get_total_weight(programs))
                .or_default()
                .push(sub);
        }

        // We'll have at most two weight values. The unbalanced program will be
        // alone, while the other group will be... a group.
        if weights.len() == 2 {
            let balanced = weights.iter().find(|(_, subs)| subs.len() > 1).unwrap();
            let unbalanced = weights.iter().find(|(_, subs)| subs.len() == 1).unwrap();
            Some((unbalanced.1.first().unwrap(), balanced.0 - unbalanced.0))
        } else {
            None
        }
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Depends if it holds subprograms or not.
        if let Some((program, subs)) = s.split_once(" -> ") {
            let name = program.chars().filter(char::is_ascii_alphabetic).collect();
            let weight = program
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            let subprograms = subs.split(',').map(|sub| sub.trim().to_string()).collect();
            Ok(Program {
                name,
                weight,
                subprograms,
            })
        } else {
            let name = s.chars().filter(char::is_ascii_alphabetic).collect();
            let weight = s
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            Ok(Program {
                name,
                weight,
                subprograms: Vec::new(),
            })
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let programs: Programs = input
        .lines()
        .map(|line| {
            let program: Program = line.parse().unwrap();
            let name = program.name.clone();
            (name, program)
        })
        .collect();

    // Before you're ready to help them, you need to make sure your information
    // is correct. What is the name of the bottom program?
    let subprograms: HashSet<&String> = programs
        .values()
        .flat_map(|program| &program.subprograms)
        .collect();
    let part1 = programs
        .keys()
        .find(|name| !subprograms.contains(name))
        .unwrap();

    println!("Part 1: {}", part1);

    // Given that exactly one program is the wrong weight, what would its weight
    // need to be to balance the entire tower?
    // The unbalanced part starts at the top (part 1). We can walk down the
    // subprograms until we no longer find unbalanced subprograms, at which
    // point we know which program needs its weight adjusted.
    // It would make sense to cache the results of the total weight calculation,
    // but mutability becomes tricky.
    let mut current = programs.get(part1).unwrap();
    let mut offset = 0;
    while let Some((program, diff)) = current.unbalanced(&programs) {
        current = program;
        offset = diff;
    }

    let part2 = current.weight + offset;
    println!("Part 2: {}", part2);

    Ok(())
}
