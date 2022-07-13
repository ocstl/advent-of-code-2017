use advent_of_code_2017::knot_hash::KnotHash;
use advent_of_code_2017::position::Position;
use std::collections::HashSet;

const INPUT: &str = "nbysizxe";
const NBR_ROWS: isize = 128;
const NBR_COLS: isize = 128;

fn main() {
    let mut disk: HashSet<Position> = (0..NBR_ROWS)
        .flat_map(|row| {
            let lengths: String = INPUT
                .chars()
                .chain(std::iter::once('-'))
                .chain(row.to_string().chars())
                .collect();
            let mut knot = KnotHash::default();
            knot.hash(lengths.bytes());
            knot.get_blocks()
                .into_iter()
                .flat_map(|value| {
                    [
                        value & 0b10000000 > 0,
                        value & 0b1000000 > 0,
                        value & 0b100000 > 0,
                        value & 0b10000 > 0,
                        value & 0b1000 > 0,
                        value & 0b100 > 0,
                        value & 0b10 > 0,
                        value & 0b1 > 0,
                    ]
                })
                .enumerate()
                .filter_map(move |(idx, used)| {
                    if used {
                        Some(Position::new(idx as isize, row))
                    } else {
                        None
                    }
                })
        })
        .collect();

    // Given your actual key string, how many squares are used?
    let part1: usize = disk.len();
    println!("Part 1: {}", part1);

    // How many regions are present given your key string?
    let mut count = 0;

    for idy in 0..NBR_ROWS {
        for idx in 0..NBR_COLS {
            let start = Position::new(idx, idy);
            if disk.remove(&start) {
                count += 1;
                let mut to_visit = vec![start];

                // We can combine checking the need to visit and removing at
                // the same time.
                while let Some(position) = to_visit.pop() {
                    to_visit.extend(
                        position
                            .adjacent()
                            .into_iter()
                            .filter(|adj| disk.remove(adj)),
                    );
                }
            }
        }
    }

    println!("Part 2: {}", count);
}
