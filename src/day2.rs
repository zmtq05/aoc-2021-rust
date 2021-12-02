use std::str::FromStr;

pub struct Command {
    directive: Direction,
    units: i32,
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let directive = match iter.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => unreachable!(),
        };
        let units = iter.next().unwrap().parse().unwrap();
        Ok(Command { directive, units })
    }
}

pub fn generator(input: &str) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    input.iter().for_each(|c| {
        match c.directive {
            Direction::Forward => horizontal += c.units,
            Direction::Down => depth += c.units,
            Direction::Up => depth -= c.units,
        };
    });

    horizontal * depth
}

pub fn part2(input: &[Command]) -> i32 {
    let (mut aim, mut horizontal, mut depth) = (0, 0, 0);

    input.iter().for_each(|c| {
        match c.directive {
            Direction::Forward => {
                horizontal += c.units;
                depth += aim * c.units;
            }
            Direction::Down => aim += c.units,
            Direction::Up => aim -= c.units,
        };
    });

    horizontal * depth
}
