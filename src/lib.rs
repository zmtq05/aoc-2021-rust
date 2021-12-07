#![allow(dead_code)]
mod day5;
mod day6;
mod day7;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day5() {
        let a = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let vents = day5::generator(a);
        let p1 = day5::part1(&vents);
        assert_eq!(p1, 5);
        let p2 = day5::part2(&vents);
        assert_eq!(p2, 12);
    }

    #[test]
    fn day6() {
        let input = "3,4,3,1,2";

        let input = day6::generator(input);
        assert_eq!(day6::part1(&input), 5934);
        assert_eq!(day6::part2(&input), 26984457539);
    }

    #[test]
    fn day7() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let input = day7::generator(input);
        assert_eq!(day7::part1(&input), 37);
    }
}
