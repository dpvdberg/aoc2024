use crate::day12::Day12;
use crate::solution::Solution;

static SAMPLE: &str = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

#[test]
fn test_part1() {
    assert_eq!(
        Day12::solve_part1(SAMPLE),
        "1930"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Day12::solve_part2(SAMPLE),
        "1206"
    );
}


#[test]
fn test_part2_ab() {
    assert_eq!(
        Day12::solve_part2(r#"
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
        "#),
        "368"
    );
}
#[test]
fn test_part2_e() {
    assert_eq!(
        Day12::solve_part2(r#"
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
        "#),
        "236"
    );
}

#[test]
fn test_part2_custom() {
    assert_eq!(
        Day12::solve_part2(r#"
        AAAAAA
        ABABBA
        AAAAAA
        "#),
        "192"
    );
}

