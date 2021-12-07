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
    (0..=max)
        .map(|i| {
            crabs
                .iter()
                .map(|&pos| {
                    let diff = if pos > i { pos - i } else { i - pos };
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
