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
