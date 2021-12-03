pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(2).unwrap()).collect())
        .collect()
}

pub fn part1(lines: &[Vec<u32>]) -> u32 {
    let modes: Vec<u32> = (0..12)
        .map(|i| get_mode(lines, i))
        .collect();
    let gamma = cal_binary_sum(&modes);
    let epsilon = cal_binary_sum(&reverse_bit(&modes));

    gamma * epsilon
}

pub fn part2(lines: &[Vec<u32>]) -> u32 {
    let oxygen = {
        let mut cloned = lines.to_vec();
        for i in 0..12 {
            let mode = get_mode(&cloned, i);
            cloned.retain(|line| line[i] == mode);
            if cloned.len() == 1 {
                break;
            }
        }
        cal_binary_sum(cloned[0].as_ref())
    };

    let co2 = {
        let mut cloned = lines.to_vec();
        for i in 0..12 {
            let mode = get_mode(&cloned, i) ^ 1;
            cloned.retain(|line| line[i] == mode);
            if cloned.len() == 1 {
                break;
            }
        }
        cal_binary_sum(cloned[0].as_ref())
    };
    oxygen * co2
}

fn get_mode(lines: &[Vec<u32>], i: usize) -> u32
{
    let zero = lines.iter().filter(|line| line[i] == 0).count();
    let one = lines.len() - zero;
    if zero > one {
        0
    } else {
        1
    }
}

fn cal_binary_sum(arr: &[u32]) -> u32 {
    arr.iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (v << (11 - i)))
}

fn reverse_bit(arr: &[u32]) -> Vec<u32> {
    arr.iter().map(|b| b ^ 1).collect()
}
