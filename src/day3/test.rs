use crate::day3::Day3;
use crate::solution::Solution;

#[test]
fn test_part1() {
    assert_eq!(
        Day3::solve_part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        "161"
    );
}



#[test]
fn test_part2() {
    assert_eq!(
        Day3::solve_part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 
        "48"
    );
}
