pub fn generator(input: &str) -> HeightMap {
    HeightMap(
        input
            .lines()
            .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
            .collect(),
    )
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct HeightMap(Vec<Vec<u32>>);

impl HeightMap {
    fn size(&self) -> (usize, usize) {
        // y size, x size
        (self.0.len(), self.0[0].len())
    }

    fn adjacents(&self, point: Point) -> Vec<Point> {
        let (ysize, xsize) = self.size();
        let mut v = vec![];
        if point.x + 1 < xsize {
            v.push((point.x + 1, point.y).into());
        }
        if point.x != 0 {
            v.push((point.x - 1, point.y).into());
        }
        if point.y + 1 < ysize {
            v.push((point.x, point.y + 1).into());
        }
        if point.y != 0 {
            v.push((point.x, point.y - 1).into());
        }
        v
    }

    fn get(&self, point: Point) -> u32 {
        self.0[point.y][point.x]
    }

    fn low_points(&self) -> Vec<Point> {
        let (ysize, xsize) = self.size();
        let mut v = vec![];
        for y in 0..ysize {
            for x in 0..xsize {
                let point = Point { x, y };
                let now = self.get(point);
                if self.adjacents(point).iter().all(|adj| self.get(*adj) > now) {
                    v.push(point);
                }
            }
        }
        v
    }
}

pub fn part1(height_map: &HeightMap) -> u32 {
    height_map
        .low_points()
        .iter()
        .fold(0, |acc, x| acc + height_map.get(*x) + 1)
}
