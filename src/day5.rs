#[derive(Clone, Debug)]
pub struct Vent {
    start: Position,
    end: Position,
}

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

pub fn generator(input: &str) -> Vec<Vent> {
    input
        .lines()
        .map(|line| line.split_once("->").unwrap())
        .map(|(start, end)| {
            let parser =
                |(x, y): (&str, &str)| (x.trim().parse().unwrap(), y.trim().parse().unwrap());
            let start = start
                .split_once(',')
                .map(parser)
                .map(|(x, y)| Position { x, y })
                .unwrap();
            let end = end
                .split_once(',')
                .map(parser)
                .map(|(x, y)| Position { x, y })
                .unwrap();
            Vent { start, end }
        })
        .collect()
}

pub fn part1(vents: &[Vent]) -> usize {
    let mut map = [[0; 1000]; 1000];
    vents
        .iter()
        .filter(|v| v.start.x == v.end.x || v.start.y == v.end.y)
        .for_each(|vent| {
            let Vent { start, end } = vent;
            let min = start.y.min(end.y);
            let max = start.y.max(end.y);
            for y in min..max + 1 {
                let min = start.x.min(end.x);
                let max = start.x.max(end.x);
                for x in min..max + 1 {
                    map[y as usize][x as usize] += 1;
                }
            }
        });

    map.iter().flatten().filter(|x| **x > 1).count()
}
