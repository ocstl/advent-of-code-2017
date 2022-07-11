const FILE: &str = "inputs/day10.txt";

const BLOCK_SIZE: usize = 256;
const NBR_ROUNDS: usize = 64;
const SUFFIX: [usize; 5] = [17, 31, 73, 47, 23];

#[derive(Debug, Clone, Copy)]
struct KnotHash {
    state: [u8; BLOCK_SIZE],
    current: usize,
    skip_size: usize,
}

impl KnotHash {
    fn round(&mut self, lengths: impl Iterator<Item = usize>) {
        // Rotate the array so that we only ever need to reverse from the start.
        // Current will keep track of where we are in the array, thus allowing
        // us to "undo" the rotation when needed.
        for length in lengths {
            self.state[0..length].reverse();
            self.state
                .rotate_left((length + self.skip_size) % BLOCK_SIZE);
            self.current = (self.current + self.skip_size + length) % BLOCK_SIZE;
            self.skip_size = (self.skip_size + 1) % BLOCK_SIZE;
        }
    }

    fn part1(&self) -> u32 {
        let mut state = self.state;
        state.rotate_right(self.current);
        u32::from(state[0]) * u32::from(state[1])
    }

    fn part2(&mut self, input: &[u8]) -> String {
        let lengths: Vec<usize> = input
            .iter()
            .copied()
            .map(usize::from)
            .chain(SUFFIX.into_iter())
            .collect();

        for _ in 0..NBR_ROUNDS {
            self.round(lengths.iter().copied());
        }

        // Undo the rotation, then convert to an hexadecimal string.
        self.state.rotate_right(self.current);
        let dense_hash = self
            .state
            .chunks_exact(16)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x));

        dense_hash.map(|x| format!("{:02x}", x)).collect()
    }
}

impl Default for KnotHash {
    fn default() -> Self {
        let mut state = [0; BLOCK_SIZE];
        for (value, slot) in (0..=BLOCK_SIZE).zip(state.iter_mut()) {
            *slot = value as u8;
        }

        KnotHash {
            state,
            current: 0,
            skip_size: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;

    // Once this process is complete, what is the result of multiplying the
    // first two numbers in the list?
    let mut knot = KnotHash::default();
    knot.round(
        input
            .split(',')
            .map(|length| length.trim().parse().unwrap()),
    );
    let part1 = knot.part1();

    println!("Part 1: {}", part1);

    // Treating your puzzle input as a string of ASCII characters, what is the
    // Knot Hash of your puzzle input? Ignore any leading or trailing
    // whitespace you might encounter.
    let mut knot = KnotHash::default();
    let part2 = knot.part2(input.trim().as_bytes());

    println!("Part 2: {}", part2);

    Ok(())
}
