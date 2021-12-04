#[derive(Clone)]
pub struct Bingo {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

type Board = Vec<i32>;

pub fn generator(input: &str) -> Bingo {
    let mut lines = input.lines();

    let count_lines = lines.clone().count();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();

    let mut boards = vec![];

    for _ in 0..(count_lines - 1) / 6 {
        let board = lines
            .by_ref()
            .skip(1)
            .take(5)
            .flat_map(|line| line.split_whitespace().map(|n| n.parse().unwrap()))
            .collect();
        boards.push(board);
    }

    Bingo { numbers, boards }
}

pub fn part1(bingo: &Bingo) -> i32 {
    let Bingo { numbers, boards } = bingo;
    let mut boards = boards.clone();

    for number in numbers {
        for board in boards.iter_mut() {
            if let Some(num) = board.iter_mut().find(|num| num == &number) {
                *num = -1;
            }
            if is_bingo(board) {
                let sum: i32 = board.iter().filter(|x| **x >= 0).sum();
                return sum * number;
            }
        }
    }

    unreachable!()
}

pub fn part2(bingo: &Bingo) -> i32 {
    let Bingo { numbers, boards } = bingo;
    let mut boards = boards.clone();
    let mut bingo_board = vec![];
    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if bingo_board.contains(&i) {
                continue;
            }
            if let Some(num) = board.iter_mut().find(|num| num == &number) {
                *num = -1;
            }
            if is_bingo(board) {
                bingo_board.push(i);
                if bingo_board.len() == 100 {
                    let sum: i32 = board.iter().filter(|x| **x >= 0).sum();
                    return number * sum;
                }
            }
        }
    }
    unreachable!()
}

fn is_bingo(board: &[i32]) -> bool {
    let mut cnt = 0;
    for y in 0..5 {
        for x in 0..5 {
            if board[y * 5 + x] < 0 {
                cnt += 1;
            }
        }
        if cnt == 5 {
            return true;
        }
        cnt = 0;
        for x in 0..5 {
            if board[x * 5 + y] < 0 {
                cnt += 1;
            }
        }
        if cnt == 5 {
            return true;
        }
        cnt = 0;
    }

    false
}
