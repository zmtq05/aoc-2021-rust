use std::{collections::HashMap, ops::Add};

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
            .filter_map(|(j, &target)| Adjacents::new(heightmap, i, j).check_lowest(target))
            .fold(0, |acc, x| acc + x + 1)
    })
}

struct Adjacents([u32; 4]); // up, down, left, right

impl Adjacents {
    fn new(heightmap: &[Vec<u32>], row: usize, col: usize) -> Self {
        let up = row
            .checked_sub(1)
            .map(|row| heightmap[row][col])
            .unwrap_or(9);
        let down = heightmap.get(row + 1).map(|v| v[col]).unwrap_or(9);
        let left = col
            .checked_sub(1)
            .map(|col| heightmap[row][col])
            .unwrap_or(9);
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

const D: [Coord; 4] = [Coord(-1, 0), Coord(1, 0), Coord(0, -1), Coord(0, 1)];

struct Basin {
    height_map: Vec<Vec<u32>>,
    visited: HashMap<Coord, bool>,
}

impl Basin {
    fn new(height_map: &[Vec<u32>]) -> Self {
        Self {
            height_map: height_map.to_vec(),
            visited: Default::default(),
        }
    }

    fn dfs(&mut self, now: Coord) -> u32 {
        let mut sum = 0;
        let entry = self.visited.entry(now).or_default();
        if *entry {
            sum += 1;
        }
        *entry = true;
        for coord in D {
            let next = now + coord;
            if next.is_out_of_range() {
                continue;
            }
            if next.get(&self.height_map) <= now.get(&self.height_map)
                || next.get(&self.height_map) == 9
            {
                continue;
            }
            sum += self.dfs(next);
        }
        sum
    }

    fn clear(&mut self) {
        self.visited.clear()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn is_out_of_range(&self) -> bool {
        self.0.is_negative() || self.1.is_negative() || self.0 >= 5 || self.1 >= 10
    }

    fn get(&self, heightmap: &[Vec<u32>]) -> u32 {
        heightmap[self.0 as usize][self.1 as usize]
    }
}

impl From<(i32, i32)> for Coord {
    fn from(pair: (i32, i32)) -> Self {
        Self(pair.0, pair.1)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn part2(heightmap: &[Vec<u32>]) -> u32 {
    let mut v = vec![];

    let mut basin = Basin::new(heightmap);
    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let now = Coord(i as i32, j as i32);
            let mut lower = true;
            for coord in D {
                let nearby = now + coord;
                if nearby.is_out_of_range() {
                    continue;
                }
                if nearby.get(heightmap) <= now.get(heightmap) {
                    lower = false;
                }
            }
            if lower {
                basin.clear();
                v.push(basin.dfs(now));
            }
        }
    }
    v.sort_unstable();
    v.reverse();
    v[0] * v[1] * v[2]
}
