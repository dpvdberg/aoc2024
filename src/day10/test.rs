use crate::day10::Day10;
use crate::solution::Solution;

#[test]
fn test_part1_simple() {
    let day = Day10 {};
    assert_eq!(
        day.solve_part1(
            r#"
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
        "#
        ),
        "2"
    );
}

#[test]
fn test_part1_crowded() {
    let day = Day10 {};
    assert_eq!(
        day.solve_part1(
            r#"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        "#
        ),
        "36"
    );
}

#[test]
fn test_part2() {
    let day = Day10 {};
    assert_eq!(
        day.solve_part2(
            r#"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"#
        ),
        "81"
    );
}
