use crate::day1::Day1;
use crate::solution::Solution;

static SAMPLE: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

#[test]
fn test_part1() {
    assert_eq!(Day1::solve_part1(SAMPLE), "11");
}

#[test]
fn test_part2() {
    assert_eq!(Day1::solve_part2(SAMPLE), "31");
}
