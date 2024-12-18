use crate::day14::Day14;
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day14 {};
    assert_eq!(
        day.solve_part1(
            r#"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
        "#
        ),
        "12"
    );
}

#[test]
fn test_part2() {
    let day = Day14 {};
    assert_eq!(day.solve_part2(""), "");
}
