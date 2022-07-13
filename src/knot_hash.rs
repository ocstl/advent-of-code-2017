use std::fmt::Display;

const BLOCK_SIZE: usize = 256;
const NBR_ROUNDS: usize = 64;
const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

#[derive(Debug, Clone, Copy)]
pub struct KnotHash {
    state: [u8; BLOCK_SIZE],
    current: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn round(&mut self, lengths: impl Iterator<Item = u8>) {
        // Rotate the array so that we only ever need to reverse from the start.
        // Current will keep track of where we are in the array, thus allowing
        // us to "undo" the rotation when needed.
        for length in lengths {
            let length = usize::from(length);
            self.state[0..length].reverse();
            self.state
                .rotate_left((length + self.skip_size) % BLOCK_SIZE);
            self.current = (self.current + self.skip_size + length) % BLOCK_SIZE;
            self.skip_size = (self.skip_size + 1) % BLOCK_SIZE;
        }
    }

    pub fn hash(&mut self, lengths: impl Iterator<Item = u8>) {
        let lengths: Vec<u8> = lengths.chain(SUFFIX.into_iter()).collect();

        for _ in 0..NBR_ROUNDS {
            self.round(lengths.iter().copied());
        }
    }

    pub fn get_state(&self) -> [u8; 256] {
        let mut state = self.state;
        state.rotate_right(self.current);
        state
    }

    pub fn get_blocks(&self) -> [u8; 16] {
        let state = self.get_state();
        let bytes = state
            .chunks_exact(16)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x));

        let mut output = [0; 16];
        for (o, b) in output.iter_mut().zip(bytes) {
            *o = b;
        }

        output
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.get_blocks()
                .into_iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>()
        )
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
