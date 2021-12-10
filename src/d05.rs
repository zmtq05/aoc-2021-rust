use std::{collections::HashMap, iter::Zip, str::FromStr};

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

type Range = Box<dyn Iterator<Item = i32>>;
impl Vent {
    fn range(&self) -> Zip<Range, Range> {
        fn range(s: i32, e: i32) -> Range {
            use std::cmp::Ordering::*;
            match s.cmp(&e) {
                Less => Box::new(s..=e),
                Equal => Box::new(std::iter::repeat(s)),
                Greater => Box::new((e..s + 1).rev()),
            }
        }
        let Self { x1, x2, y1, y2 } = *self;
        let x = range(x1, x2);
        let y = range(y1, y2);
        x.zip(y)
    }
}

pub fn generator(input: &str) -> Vec<Vent> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(vents: &[Vent]) -> usize {
    let mut map = HashMap::new();
    vents
        .iter()
        .filter(|v| v.x1 == v.x2 || v.y1 == v.y2)
        .for_each(|v| v.range().for_each(|xy| *map.entry(xy).or_insert(0) += 1));
    map.values().filter(|x| **x > 1).count()
}

pub fn part2(vents: &[Vent]) -> usize {
    let mut map = HashMap::new();
    vents
        .iter()
        .for_each(|v| v.range().for_each(|xy| *map.entry(xy).or_insert(0) += 1));
    map.values().filter(|x| **x > 1).count()
}
