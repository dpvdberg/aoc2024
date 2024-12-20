use crate::day20::Day20;
use crate::solution::Solution;

static SAMPLE: &str = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

#[test]
fn test_part1() {
    let day = Day20 {};
    assert_eq!(day.solve_part1(SAMPLE), "");
}

#[test]
fn test_part2() {
    let day = Day20 {};
    assert_eq!(day.solve_part2(""), "");
}
