use crate::day17::Day17;
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day17 {};
    assert_eq!(
        day.solve_part1(
            r#"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        "#
        ),
        "4,6,3,5,6,3,5,2,1,0"
    );
}

#[test]
fn test_part2() {
    let day = Day17 {};
    assert_eq!(
        day.solve_part2(
            r#"
        Register A: 2024
        Register B: 0
        Register C: 0
        
        Program: 0,3,5,4,3,0
        "#
        ),
        "117440"
    );
}
