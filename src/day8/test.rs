use crate::day8::Day8;
use crate::solution::Solution;


static SAMPLE : &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

#[test]
fn test_part1() {
    assert_eq!(
        Day8::solve_part1(SAMPLE),
        "14"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Day8::solve_part2(SAMPLE),
        "34"
    );
}
