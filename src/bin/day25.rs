use std::collections::{HashMap, HashSet};

const FILE: &str = "inputs/day25.txt";

type Rule = (usize, isize, char);
type State = [Rule; 2];

fn parse_input(s: &str) -> (char, usize, HashMap<char, State>) {
    let mut iter = s.lines();

    let state = iter
        .next()
        .unwrap()
        .trim_end_matches('.')
        .chars()
        .last()
        .expect("Missing starting state.");
    let steps = iter
        .next()
        .unwrap()
        .split_whitespace()
        .find_map(|word| word.parse::<usize>().ok())
        .expect("Missing number of steps.");

    let mut states = HashMap::new();

    while iter.next().is_some() {
        let name = iter
            .next()
            .unwrap()
            .trim_end_matches(':')
            .chars()
            .last()
            .expect("Missing rule name.");
        // Rule for 0.
        let _ = iter.next();
        let value = iter
            .next()
            .unwrap()
            .trim_end_matches('.')
            .split_whitespace()
            .find_map(|word| word.parse::<usize>().ok())
            .expect("Missing value.");
        let direction = match iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .expect("Missing direction.")
        {
            "right." => 1,
            "left." => -1,
            _ => panic!("Invalid direction."),
        };
        let next_state = iter
            .next()
            .unwrap()
            .trim_end_matches('.')
            .chars()
            .last()
            .expect("Missing next state.");

        let rule_0 = (value, direction, next_state);

        // Rule for 0.
        let _ = iter.next();
        let value = iter
            .next()
            .unwrap()
            .trim_end_matches('.')
            .split_whitespace()
            .find_map(|word| word.parse::<usize>().ok())
            .expect("Missing value.");
        let direction = match iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .expect("Missing direction.")
        {
            "right." => 1,
            "left." => -1,
            _ => panic!("Invalid direction."),
        };
        let next_state = iter
            .next()
            .unwrap()
            .trim_end_matches('.')
            .chars()
            .last()
            .expect("Missing next state.");

        let rule_1 = (value, direction, next_state);

        states.insert(name, [rule_0, rule_1]);
    }

    (state, steps, states)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    let (mut state, steps, states) = parse_input(&input);

    let mut tape = HashSet::new();
    let mut position = 0;
    for _ in 0..steps {
        let (value, direction, next_state) =
            states.get(&state).unwrap()[if tape.contains(&position) { 1 } else { 0 }];
        if value == 1 {
            tape.insert(position);
        } else {
            tape.remove(&position);
        }

        position += direction;
        state = next_state;
    }

    let part1 = tape.len();
    println!("Part 1: {}", part1);

    Ok(())
}
