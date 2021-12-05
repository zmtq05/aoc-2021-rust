#[allow(dead_code)]
mod day5;
#[cfg(test)]
mod test {
    use crate::day5;
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
}