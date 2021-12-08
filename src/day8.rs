use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|pair| {
            (
                pair.0.split_whitespace().collect(),
                pair.1.split_whitespace().collect(),
            )
        })
        .collect()
}

pub fn part1(entries: &[(Vec<&str>, Vec<&str>)]) -> usize {
    entries
        .iter()
        .map(|pair| &pair.1)
        .flatten()
        .filter(|s| {
            let len = s.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count()
}

pub fn part2(entries: &[(Vec<&str>, Vec<&str>)]) -> usize {
    entries.iter().fold(0, |acc, (wires, outputs)| {
        let decoder = Decoder::new(wires);
        acc + outputs
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, output)| {
                acc + decoder.decode(output) * 10usize.pow(i as u32)
            })
    })
}

struct Decoder {
    one: HashSet<char>,
    four: HashSet<char>,
}

impl Decoder {
    fn new(wires: &[&str]) -> Self {
        fn seg_chars_to_set(wires: &[&str], len: usize) -> HashSet<char> {
            wires
                .iter()
                .filter(|wire| wire.len() == len)
                .take(1)
                .next()
                .unwrap()
                .chars()
                .collect()
        }
        let one = seg_chars_to_set(wires, 2);
        let four = seg_chars_to_set(wires, 4);

        Self { one, four }
    }

    fn int1count(&self, other: &str) -> usize {
        self.one.intersection(&other.chars().collect()).count()
    }

    fn int4count(&self, other: &str) -> usize {
        self.four.intersection(&other.chars().collect()).count()
    }

    fn decode(&self, other: &str) -> usize {
        match other.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 if self.int1count(other) == 2 => 3,
            5 if self.int4count(other) == 3 => 5,
            5 => 2,
            6 if self.int1count(other) == 1 => 6,
            6 if self.int4count(other) == 4 => 9,
            6 => 0,
            7 => 8,
            _ => unreachable!(),
        }
    }
}
