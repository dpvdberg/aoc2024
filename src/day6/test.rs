use crate::day6::Day6;
use crate::solution::Solution;

static SAMPLE : &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

#[test]
fn test_part1() {
    assert_eq!(
        Day6::solve_part1(SAMPLE),
        "41"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Day6::solve_part2(SAMPLE),
        "6"
    );
}
