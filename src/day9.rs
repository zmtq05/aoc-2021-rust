pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part1(heightmap: &[Vec<u32>]) -> u32 {
    heightmap.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate()
            .filter_map(|(j, &target)| Adjacents::new(heightmap, i, j).check_lowest(target))
            .fold(0, |acc, x| acc + x + 1)
    })
}

struct Adjacents([u32; 4]); // up, down, left, right

impl Adjacents {
    fn new(heightmap: &[Vec<u32>], row: usize, col: usize) -> Self {
        let up = row.checked_sub(1).map(|row| heightmap[row][col]).unwrap_or(9);
        let down = heightmap.get(row + 1).map(|v| v[col]).unwrap_or(9);
        let left = col.checked_sub(1).map(|col| heightmap[row][col]).unwrap_or(9);
        let right = *heightmap[row].get(col + 1).unwrap_or(&9);
        Self([up, down, left, right])
    }

    fn check_lowest(&self, target: u32) -> Option<u32> {
        self.0
            .as_ref()
            .iter()
            .all(|adj| target < *adj)
            .then(|| target)
    }
}
