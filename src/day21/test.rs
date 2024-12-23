use crate::day21::{Day21};
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day21 {};
    assert_eq!(
        day.solve_part1(r#"
        029A
        980A
        179A
        456A
        379A
        "#),
        "126384"
    );
}