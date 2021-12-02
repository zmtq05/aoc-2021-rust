use std::{num::ParseIntError, str::FromStr};

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, units) = s.split_once(' ').unwrap();
        let units = units.parse()?;

        Ok(match direction {
            "forward" => Self::Forward(units),
            "down" => Self::Down(units),
            "up" => Self::Up(units),
            _ => unreachable!(),
        })
    }
}

pub fn generator(input: &str) -> Vec<Direction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[Direction]) -> i32 {
    let (h, d) = input
        .iter()
        .fold((0, 0), |(h, d), direction| match direction {
            Direction::Forward(units) => (h + units, d),
            Direction::Down(units) => (h, d + units),
            Direction::Up(units) => (h, d - units),
        });
    h * d
}

pub fn part2(input: &[Direction]) -> i32 {
    let (h, d, _) = input
        .iter()
        .fold((0, 0, 0), |(h, d, aim), direction| match direction {
            Direction::Forward(units) => (h + units, d + units * aim, aim),
            Direction::Down(units) => (h, d, aim + units),
            Direction::Up(units) => (h, d, aim - units),
        });
    h * d
}
