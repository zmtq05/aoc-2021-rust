use std::ops::Sub;

pub fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|a| a.parse().unwrap()).collect()
}

pub fn part1(crabs: &[i32]) -> i32 {
    let median = {
        let mut v = crabs.to_vec();
        v.sort_unstable();
        let mid = v.len() / 2;
        v[mid]
    };

    crabs.iter().map(|pos| median.sub(pos).abs()).sum()
}

pub fn part2(crabs: &[i32]) -> i32 {
    let max = *crabs.iter().max().unwrap();
    (0..=max)
        .map(|i| {
            crabs
                .iter()
                .map(|pos| i.sub(pos).abs())
                .map(|diff| diff * (diff + 1) / 2)
                .sum()
        })
        .min()
        .unwrap()
}
