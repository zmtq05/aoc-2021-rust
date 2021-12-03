pub fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    let mut store = [0; 12];
    for line in input {
        for (i, b) in line.chars().enumerate() {
            store[i] += match b {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            }
        }
    }
    let mode: Vec<_> = store
        .iter()
        .map(|&b| if b as usize > input.len() / 2 { 1 } else { 0 })
        .collect();

    let gamma = {
        let gamma = mode.iter().map(|b| b.to_string()).collect::<String>();
        u32::from_str_radix(&gamma, 2).unwrap()
    };

    let epsilon = {
        let e: String = mode
            .iter()
            .map(|&b| if b == 1 { 0 } else { 1 })
            .map(|b| b.to_string())
            .collect();
        u32::from_str_radix(&e, 2).unwrap()
    };

    gamma * epsilon
}
