pub fn generator(input: &str) -> Vec<u8> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub fn part1(input: &[u8]) -> usize {
    let mut fishes = [0; 9];
    input.iter().for_each(|x| fishes[*x as usize] += 1);

    for _ in 0..80 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}

pub fn part2(input: &[u8]) -> usize {
    let mut fishes = [0; 9];
    input.iter().for_each(|x| fishes[*x as usize] += 1);

    for _ in 0..256 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}
