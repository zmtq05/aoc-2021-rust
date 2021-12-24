#![allow(dead_code)]
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

#[cfg(test)]
mod test {
    macro_rules! parse_input {
        ($day:ident) => {{
            let input =
                include_str!(concat!("../input/2021/small/", stringify!($day), ".txt")).trim();
            $day::generator(input)
        }};
    }

    macro_rules! test_small {
        ($day:ident; $($part:ident => $expected:expr;)*) => {{
            let input = parse_input!($day);
            $(assert_eq!($day::$part(&input), $expected);)*
        }};
        ($day:ident; $x:expr, $x2:expr$(,)?) => {{
            let input = parse_input!($day);
            assert_eq!($day::part1(&input), $x);
            assert_eq!($day::part2(&input), $x2);
        }};
        ($day:ident; $expected:expr$(,)?) => {{
            let input = parse_input!($day);
            assert_eq!($day::part1(&input), $expected);
        }};
    }

    use super::*;
    #[test]
    fn day05() {
        test_small! {
            day05;
            5,
            12,
        }
    }

    #[test]
    fn day06() {
        test_small! {
            day06;
            5934,
            26984457539,
        }
    }

    #[test]
    fn day07() {
        test_small! {
            day07;
            37,
            168,
        }
    }

    #[test]
    fn day08() {
        test_small! {
            day08;
            26,
            61229,
        }
    }

    #[test]
    fn day09() {
        test_small! {
            day09;
            15,
            1134,
        }
    }

    #[test]
    fn day10() {
        test_small! {
            day10;
            26397,
            288957,
        }
    }
}
