use crate::day6::Day6;
use crate::solution::Solution;

static SAMPLE: &str = r#"
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
    let day = Day6 {};
    assert_eq!(day.solve_part1(SAMPLE), "41");
}

#[test]
fn test_part2() {
    let day = Day6 {};
    assert_eq!(day.solve_part2(SAMPLE), "6");
}
