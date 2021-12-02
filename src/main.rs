#![allow(dead_code)]

aoc_main::main! {
    year 2021;

    day1 : generator => part1, part2, part1_v2, part2_v2;
    day2 : generator => part1, part2;
}

mod day1;
mod day2;
