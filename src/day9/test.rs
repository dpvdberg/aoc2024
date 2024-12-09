use crate::day9::Day9;
use crate::solution::Solution;


#[test]
fn test_part1() {
    assert_eq!(
        Day9::solve_part1("2333133121414131402"),
        "1928"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Day9::solve_part2("2333133121414131402"),
        "2858"
    );
}
