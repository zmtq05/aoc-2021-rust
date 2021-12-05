use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

#[derive(Clone, Debug)]
pub struct Vent {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl FromStr for Vent {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (xy1, xy2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = xy1
            .split_once(',')
            .map(|xy| (xy.0.parse().unwrap(), xy.1.parse().unwrap()))
            .unwrap();
        let (x2, y2) = xy2
            .split_once(',')
            .map(|xy| (xy.0.parse().unwrap(), xy.1.parse().unwrap()))
            .unwrap();
        Ok(Self { x1, x2, y1, y2 })
    }
}
pub fn generator(input: &str) -> Vec<Vent> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

enum Slope {
    Horizontal(i32, RangeInclusive<i32>),
    Vertical(i32, RangeInclusive<i32>),
    Diagonal(RangeInclusive<i32>, RangeInclusive<i32>),
}

impl From<&Vent> for Slope {
    fn from(vent: &Vent) -> Self {
        let Vent { x1, x2, y1, y2 } = *vent;
        let x_range = if x1 > x2 { x2..=x1 } else { x1..=x2 };
        let y_range = if y1 > y2 { y2..=y1 } else { y1..=y2 };
        if x1 == x2 {
            Slope::Vertical(x1, y_range)
        } else if y1 == y2 {
            Slope::Horizontal(y1, x_range)
        } else {
            Slope::Diagonal(x_range, y_range)
        }
    }
}

pub fn part1(vents: &[Vent]) -> usize {
    let mut map = HashMap::new();
    vents.iter().for_each(|vent| {
        match Slope::from(vent) {
            Slope::Horizontal(x, y_range) => y_range
                .into_iter()
                .for_each(|y| *map.entry((x, y)).or_insert(0) += 1),
            Slope::Vertical(y, x_range) => x_range
                .into_iter()
                .for_each(|x| *map.entry((x, y)).or_insert(0) += 1),
            Slope::Diagonal(_, _) => {}
        };
    });

    map.values().filter(|x| **x > 1).count()
}
