pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part1(heightmap: &[Vec<u32>]) -> u32 {
    heightmap.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter_map(|(j, col)| Adjacent::new(heightmap, j, i).is_lowest())
            .sum::<u32>()
    })
}

struct Adjacent {
    target: u32,
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

impl Adjacent {
    fn new(heightmap: &[Vec<u32>], col: usize, row: usize) -> Self {
        let target = heightmap[row][col];
        let up = {
            if let Some(i) = row.checked_sub(1) {
                heightmap[i][col]
            } else {
                9
            }
        };
        let down = heightmap.get(row + 1).map(|v| v[col]).unwrap_or(9);
        let left = if let Some(col) = col.checked_sub(1) {
            heightmap[row][col]
        } else {
            9
        };
        let right = *heightmap[row].get(col + 1).unwrap_or(&9);
        Self {
            target,
            up,
            down,
            left,
            right,
        }
    }

    fn is_lowest(&self) -> Option<u32> {
        [self.up, self.down, self.left, self.right]
            .into_iter()
            .all(|adj| self.target < adj)
            .then(|| self.target + 1)
    }
}
