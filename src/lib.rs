#![allow(dead_code)]
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

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
        ($day:ident; $expected:expr, $expected2:expr$(,)?) => {{
            let input = parse_input!($day);
            assert_eq!($day::part1(&input), $expected);
            assert_eq!($day::part2(&input), $expected2);
        }};
        ($day:ident; $expected:expr$(,)?) => {{
            let input = parse_input!($day);
            assert_eq!($day::part1(&input), $expected);
        }};
    }

    use super::*;
    #[test]
    fn d05() {
        test_small! {
            d05;
            5,
            12,
        }
    }

    #[test]
    fn d06() {
        test_small! {
            d06;
            5934,
            26984457539,
        }
    }

    #[test]
    fn d07() {
        test_small! {
            d07;
            37,
            168,
        }
    }

    #[test]
    fn d08() {
        test_small! {
            d08;
            26,
            61229,
        }
    }

    #[test]
    fn d09() {
        test_small! {
            d09;
            15,
            1134,
        }
    }

    #[test]
    fn d10() {
        test_small! {
            d10;
            26397,
            288957,
        }
    }
}
