use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|a| a.parse().unwrap()).collect()
}

pub fn part1(crabs: &[u32]) -> u32 {
    let median = {
        let mut v = crabs.to_vec();
        v.sort_unstable();
        let mid = v.len() / 2;
        v[mid]
    };

    crabs
        .iter()
        .map(|&crab| {
            if crab > median {
                crab - median
            } else {
                median - crab
            }
        })
        .sum()
}

pub fn part2(crabs: &[u32]) -> u32 {
    let max = *crabs.iter().max().unwrap();

    let mut fuels = HashMap::new();

    (0..=max).for_each(|x| {
        crabs.iter().for_each(|&crab| {
            let fuel = fuels.entry(x).or_default();
            let diff = if crab > x { crab - x } else { x - crab };
            *fuel += diff * (diff + 1) / 2;
        })
    });

    *fuels.values().min().unwrap()
}
