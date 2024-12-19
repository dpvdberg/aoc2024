use crate::day19::Day19;
use crate::solution::Solution;


static SAMPLE: &str = r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

#[test]
fn test_part1() {
    let day = Day19 {};
    assert_eq!(day.solve_part1(SAMPLE), "6");
}

#[test]
fn test_part2() {
    let day = Day19 {};
    assert_eq!(day.solve_part2(SAMPLE), "16");
}
