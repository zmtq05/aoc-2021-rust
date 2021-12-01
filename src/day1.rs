pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }
    count
}

pub fn part1_v2(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

pub fn part2(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        let a = input[i] + input[i + 1] + input[i + 2];
        let b = input[i + 1] + input[i + 2] + input[i + 3];
        if b > a {
            count += 1
        }
    }
    count
}

pub fn part2_v2(input: &[usize]) -> usize {
    let windows: Vec<usize> = input.windows(3).map(|window| window.iter().sum()).collect();
    part1_v2(&windows)
}
