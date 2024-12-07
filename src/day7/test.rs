use crate::day7::Day7;
use crate::solution::Solution;

static SAMPLE: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;


#[test]
fn test_part1() {
    assert_eq!(
        Day7::solve_part1(SAMPLE),
        "3749"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Day7::solve_part2(SAMPLE),
        "11387"
    );
}
