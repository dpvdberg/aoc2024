use crate::day2::Day2;
use crate::solution::Solution;

static SAMPLE: &str = r#"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    "#;

#[test]
fn test_part1() {
    let day = Day2 {};
    assert_eq!(day.solve_part1(SAMPLE), "2");
}

#[test]
fn test_part2() {
    let day = Day2 {};
    assert_eq!(day.solve_part2(SAMPLE), "4");
}
