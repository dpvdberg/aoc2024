use crate::day18::Day18;
use crate::solution::Solution;

static SAMPLE: &str = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;

#[test]
fn test_part1() {
    let day = Day18 {
        max_byte_count: Some(12)
    };
    assert_eq!(day.solve_part1(SAMPLE), "22");
}

#[test]
fn test_part2() {
    let day = Day18 {
        max_byte_count: None
    };
    assert_eq!(day.solve_part2(SAMPLE), "6,1");
}
