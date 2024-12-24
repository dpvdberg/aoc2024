use crate::day24::Day24;
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day24 {};
    assert_eq!(day.solve_part1(r#"
    x00: 1
    x01: 1
    x02: 1
    y00: 0
    y01: 1
    y02: 0
    
    x00 AND y00 -> z00
    x01 XOR y01 -> z01
    x02 OR y02 -> z02
    "#), "4");
}

#[test]
fn test_part2() {
    let day = Day24 {};
    assert_eq!(day.solve_part2(""), "");
}
