use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|a| a.parse().unwrap()).collect()
}

pub fn part1(crabs: &[u32]) -> u32 {
    let mut map = HashMap::new();

    crabs
        .iter()
        .for_each(|crab| *map.entry(*crab).or_insert(0) += 1);

    let &mode = map.iter().max_by_key(|x| x.1).unwrap().0;

    let mut fuel = 0;

    crabs.iter().for_each(|&crab| {
        fuel += if crab > mode {
            crab - mode
        } else {
            mode - crab
        }
    });
    fuel
}
