use advent_of_code_2017::position::{Direction, Position};
use std::collections::HashMap;

const INPUT: u32 = 361527;

fn main() {
    // How many steps are required to carry the data from the square identified
    // in your puzzle input all the way to the access port?
    // We can generate the "center" squares (counting the steps), then find the
    // minimum distance to reach one of them.
    let mut centers = [1, 1, 1, 1];
    let mut step = 1;
    let mut count = 0;

    while INPUT > centers[3] {
        count += 1;
        for center in centers.iter_mut() {
            *center += step;
            step += 2;
        }
    }

    let part1 = count
        + centers
            .iter()
            .map(|c| c.abs_diff(INPUT))
            .min()
            .unwrap_or_default();
    println!("Part 1: {}", part1);

    // It may be possible to do this mathematically, but, given the target,
    // iteration seems simpler.
    let mut grid: HashMap<Position, u32> = HashMap::default();
    grid.insert(Position::default(), 1);
    let mut steps = 1;
    let mut direction = Direction::Right;
    let mut cursor = Position::default();

    // The cursor will walk X steps, turn left, walk again X steps, turn left,
    // walk X + 1 steps, turn left, walk X + 1 steps, turn left, walk X + 2
    // steps, etc.
    // We can easily do this with nested loops, but breaking out requires a
    // label (which I had never used before!).
    'outer: loop {
        for _ in 0..steps {
            cursor = cursor + direction;
            let value = cursor
                .neighbours()
                .iter()
                .map(|p| grid.get(p).copied().unwrap_or_default())
                .sum();

            if value > INPUT {
                println!("Part 2: {}", value);
                break 'outer;
            }
            grid.insert(cursor, value);
        }
        direction = direction.turn_left();

        for _ in 0..steps {
            cursor = cursor + direction;
            let value = cursor
                .neighbours()
                .iter()
                .map(|p| grid.get(p).copied().unwrap_or_default())
                .sum();
            if value > INPUT {
                println!("Part 2: {}", value);
                break;
            }
            grid.insert(cursor, value);
        }
        direction = direction.turn_left();
        steps += 1;
    }
}
