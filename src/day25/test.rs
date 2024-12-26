use crate::day25::Day25;
use crate::solution::Solution;

static SAMPLE: &str = r#"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

#[test]
fn test_part1() {
    let day = Day25 {};
    assert_eq!(day.solve_part1(SAMPLE), "3");
}

#[test]
fn test_part2() {
    let day = Day25 {};
    assert_eq!(day.solve_part2(""), "");
}
