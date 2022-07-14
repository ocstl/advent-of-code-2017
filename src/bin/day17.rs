use std::collections::VecDeque;

const INPUT: usize = 380;
const TARGET: usize = 2017;
const TARGET2: usize = 50_000_000;

fn main() {
    // What is the value after 2017 in your completed circular buffer?
    let mut buffer = VecDeque::with_capacity(TARGET2 + 1);
    buffer.push_front(0);

    for value in 1..=TARGET {
        // +1 to align with the front of the buffer.
        buffer.rotate_left((INPUT + 1) % buffer.len());
        buffer.push_front(value);
    }

    buffer.rotate_left(1);
    println!("Part 1: {}", buffer.front().unwrap());

    // What is the value after 0 the moment 50000000 is inserted?
    // Alternatively, we could simply keep track when the current position
    // falls on 0 (index 0), and keep track of the most recently value inserted
    // after it, but this is reasonably fast.
    buffer.rotate_right(1);
    for value in (TARGET + 1)..=TARGET2 {
        // Since we know the buffer is long enough, we no longer need the
        // modulus.
        buffer.rotate_left(INPUT + 1);
        buffer.push_front(value);
    }

    // Use the `or` just in case the last element is 0, in which case we want
    // the front element.
    let part2 = buffer
        .iter()
        .skip_while(|&&value| value != 0)
        .nth(1)
        .or_else(|| buffer.front())
        .unwrap();
    println!("Part 2: {}", part2);
}
