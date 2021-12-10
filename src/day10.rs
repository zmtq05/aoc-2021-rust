pub fn generator(input: &str) -> Vec<Vec<Bracket>> {
    input
        .lines()
        .map(|line| line.chars().map(Bracket::from).collect())
        .collect()
}

pub fn part1(lines: &[Vec<Bracket>]) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mut open_brackets: Vec<Kind> = vec![];

        for bracket in line {
            match bracket.state {
                State::Open => open_brackets.push(bracket.kind.clone()),
                State::Close => if open_brackets.pop().unwrap() != bracket.kind {
                    sum += bracket.kind.point();
                    break;
                }
            }
        }
    }

    sum
}

// pub fn part2(lines: &[VecDeque<char>]) -> u64 {
//     let mut scores = vec![];
//     'outer: for mut line in lines.to_vec() {
//         let mut sum = 0;
//         let mut states = vec![];
//         while let Some(bracket) = line.pop_front() {
//             let state = State::from(bracket);
//             match &state {
//                 State::Open(_) => states.push(state),
//                 State::Close(bracket) => {
//                     if let Some(last) = states.last() {
//                         match last {
//                             State::Open(last_bracket) if last_bracket == bracket => {
//                                 states.pop();
//                             }
//                             _ => continue 'outer,
//                         }
//                     } else {
//                         continue 'outer;
//                     }
//                 }
//             }
//         }

//         for state in states.into_iter().rev() {
//             match state {
//                 State::Open(bracket) => {
//                     sum *= 5;
//                     sum += bracket.point2();
//                 }
//                 State::Close(_) => unreachable!(),
//             }
//         }

//         scores.push(sum);
//     }
//     scores.sort_unstable();
//     scores[scores.len() / 2]
// }

#[derive(Clone)]
pub struct Bracket {
    kind: Kind,
    state: State,
}

impl From<char> for Bracket {
    fn from(char: char) -> Self {
        use Kind::*;
        use State::*;
        match char {
            '(' => Self {
                kind: Parentheses,
                state: Open,
            },
            ')' => Self {
                kind: Parentheses,
                state: Close,
            },
            '{' => Self {
                kind: Curly,
                state: Open,
            },
            '}' => Self {
                kind: Curly,
                state: Close,
            },
            '[' => Self {
                kind: Square,
                state: Open,
            },
            ']' => Self {
                kind: Square,
                state: Close,
            },
            '<' => Self {
                kind: Angle,
                state: Open,
            },
            '>' => Self {
                kind: Angle,
                state: Close,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Kind {
    Parentheses,
    Square,
    Curly,
    Angle,
}

impl Kind {
    fn point(&self) -> u32 {
        match self {
            Kind::Parentheses => 3,
            Kind::Square => 57,
            Kind::Curly => 1197,
            Kind::Angle => 25137,
        }
    }

    fn point2(&self) -> u64 {
        match self {
            Kind::Parentheses => 1,
            Kind::Square => 2,
            Kind::Curly => 3,
            Kind::Angle => 4,
        }
    }
}

#[derive(Clone)]
pub enum State {
    Open,
    Close,
}
