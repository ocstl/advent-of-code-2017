use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day20.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Position { x, y, z }
    }

    pub fn distance_to_origin(self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::ops::Add<Velocity> for Position {
    type Output = Self;

    fn add(self, rhs: Velocity) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

impl Velocity {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Velocity { x, y, z }
    }
}

impl std::ops::Add<Acceleration> for Velocity {
    type Output = Self;

    fn add(self, rhs: Acceleration) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Acceleration {
    x: isize,
    y: isize,
    z: isize,
}

impl Acceleration {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Acceleration { x, y, z }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl Particle {
    pub fn distance_to_origin(&self) -> isize {
        self.position.distance_to_origin()
    }

    pub fn step(self) -> Self {
        let velocity = self.velocity + self.acceleration;
        let position = self.position + velocity;

        Self {
            position,
            velocity,
            acceleration: self.acceleration,
        }
    }
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .chars()
            .filter(|&c| c.is_ascii_digit() || c == '-' || c == ',')
            .collect::<String>();
        let mut iter = s.split(',');

        let position = Position::new(
            iter.next()
                .expect("Missing x.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing y.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing z.")
                .parse()
                .expect("Bad coordinate."),
        );

        let velocity = Velocity::new(
            iter.next()
                .expect("Missing x.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing y.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing z.")
                .parse()
                .expect("Bad coordinate."),
        );

        let acceleration = Acceleration::new(
            iter.next()
                .expect("Missing x.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing y.")
                .parse()
                .expect("Bad coordinate."),
            iter.next()
                .expect("Missing z.")
                .parse()
                .expect("Bad coordinate."),
        );

        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let particles: Vec<Particle> = input.lines().map(|line| line.parse().unwrap()).collect();

    // Which particle will stay closest to position <0,0,0> in the long term?
    // We'll use simulation to do this. 1000 steps seems sufficient in this
    // case.
    let mut part1 = particles.clone();
    for _ in 0..1000 {
        part1 = part1.into_iter().map(Particle::step).collect();
    }

    let part1 = part1
        .into_iter()
        .enumerate()
        .min_by_key(|(_, p)| p.distance_to_origin())
        .unwrap()
        .0;
    println!("Part 1: {}", part1);

    // How many particles are left after all collisions are resolved?
    // Again, simulation. Though we could try to find a longest time to
    // collision using some math, 1000 steps is not that bad.
    // Using a `HashMap` allows us to count the number in a given position; at
    // the next step, we need only retain the particle all by their lonesome.
    let mut part2: HashMap<Position, Vec<Particle>> = HashMap::new();
    for particle in particles {
        part2.entry(particle.position).or_default().push(particle);
    }

    for _ in 0..1000 {
        let mut temp: HashMap<Position, Vec<Particle>> = HashMap::new();
        for (_, v) in part2 {
            if v.len() == 1 {
                for particle in v {
                    let particle = particle.step();
                    temp.entry(particle.position).or_default().push(particle);
                }
            }
        }
        part2 = temp;
    }

    println!("Part 2: {}", part2.len());

    Ok(())
}
