use std::collections::HashSet;

pub fn generator(input: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let split_iter = input.lines().map(|l| l.split_once(" | ").unwrap());

    let a = split_iter
        .clone()
        .map(|x| x.0.split_whitespace().collect())
        .collect();
    let b = split_iter
        .map(|x| x.1.split_whitespace().collect())
        .collect();
    (a, b)
}

pub fn part1((_, outputs): &(Vec<Vec<&str>>, Vec<Vec<&str>>)) -> usize {
    outputs
        .iter()
        .flatten()
        .filter(|s| {
            let len = s.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count()
}

pub fn part2((wire_lines, output_lines): &(Vec<Vec<&str>>, Vec<Vec<&str>>)) -> usize {
    let mut sum = 0;
    for (wires, outputs) in wire_lines.iter().zip(output_lines) {
        let decoder = Decoder::new(wires);
        for (i, output) in outputs.iter().rev().enumerate() {
            // let output = outputs[i];
            let segment = decoder.decode(output);
            sum += segment * 10usize.pow(i as u32);
        }
    }
    sum
}

struct Decoder {
    one: HashSet<char>,
    four: HashSet<char>,
}

impl Decoder {
    fn new(wires: &[&str]) -> Self {
        fn rename_me(wires: &[&str], len: usize) -> HashSet<char> {
            wires
                .iter()
                .filter(|wire| wire.len() == len)
                .take(1)
                .next()
                .unwrap()
                .chars()
                .collect()
        }
        let one = rename_me(wires, 2);
        let four = rename_me(wires, 4);

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
