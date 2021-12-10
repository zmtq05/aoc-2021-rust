use std::collections::VecDeque;

pub fn generator(input: &str) -> Vec<VecDeque<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(lines: &[VecDeque<char>]) -> u32 {
    let mut sum = 0;
    for mut line in lines.to_vec() {
        let mut states = vec![];
        while let Some(bracket) = line.pop_front() {
            let state = State::from(bracket);
            match &state {
                State::Open(_) => states.push(state),
                State::Close(bracket) => if !rename_me(&mut states, bracket, &mut sum) {
                    break;
                },
            };
        }
    }

    sum
}

fn rename_me(states: &mut Vec<State>, bracket: &Bracket, sum: &mut u32) -> bool {
    if let Some(last) = states.last() {
        if let State::Open(last_bracket) = last {
            if last_bracket == bracket {
                states.pop();
                return true;
            } else {
                *sum += bracket.point();
            }
        }
    } else {
        *sum += bracket.point();
    }
    false
}

#[derive(PartialEq, Eq)]
enum Bracket {
    Parentheses,
    Curly,
    Square,
    Angle,
}

impl Bracket {
    fn point(&self) -> u32 {
        match self {
            Bracket::Parentheses => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137
        }
    }
}

impl From<char> for State {
    fn from(char: char) -> Self {
        use Bracket::*;
        use State::*;
        match char {
            '(' => Open(Parentheses),
            ')' => Close(Parentheses),
            '{' => Open(Curly),
            '}' => Close(Curly),
            '[' => Open(Square),
            ']' => Close(Square),
            '<' => Open(Angle),
            '>' => Close(Angle),
            _ => unreachable!()
        }
    }
}



enum State {
    Open(Bracket),
    Close(Bracket),
}