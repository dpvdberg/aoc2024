use crate::day5::Day5;
use crate::solution::Solution;

static SAMPLE: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

#[test]
fn test_part1() {
    let day = Day5 {};
    assert_eq!(day.solve_part1(SAMPLE), "143");
}

#[test]
fn test_part2() {
    let day = Day5 {};
    assert_eq!(day.solve_part2(SAMPLE), "123");
}
