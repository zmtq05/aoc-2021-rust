#![allow(dead_code)]
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

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
        let vents = d05::generator(a);
        let p1 = d05::part1(&vents);
        assert_eq!(p1, 5);
        let p2 = d05::part2(&vents);
        assert_eq!(p2, 12);
    }

    #[test]
    fn day6() {
        let input = "3,4,3,1,2";

        let input = d06::generator(input);
        assert_eq!(d06::part1(&input), 5934);
        assert_eq!(d06::part2(&input), 26984457539);
    }

    #[test]
    fn day7() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let input = d07::generator(input);
        assert_eq!(d07::part1(&input), 37);
        assert_eq!(d07::part2(&input), 168);
    }

    #[test]
    fn day8() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input = d08::generator(input);
        assert_eq!(d08::part1(&input), 26);
        assert_eq!(d08::part2(&input), 61229);
    }

    #[test]
    fn day9() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let input = d09::generator(input);
        assert_eq!(d09::part1(&input), 15);
        assert_eq!(d09::part2(&input), 1134);
    }

    #[test]
    fn day10() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let input = d10::generator(input);
        assert_eq!(d10::part1(&input), 26397);
        assert_eq!(d10::part2(&input), 288957);
    }
}
