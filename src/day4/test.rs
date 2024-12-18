use crate::day4::Day4;
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part1(
            r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#
        ),
        "18"
    );
}

#[test]
fn test_part1_non_square() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part1(
            r#"
        ..X.
        ..M.
        ..A.
        ..S.
        ....
        ....
        ....
        "#
        ),
        "1"
    );
}

#[test]
fn test_part2() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part2(
            r#"
        .M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        ..........
        "#
        ),
        "9"
    );
}

#[test]
fn test_part2_simple_updown() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part2(
            r#"
        S.S
        .A.
        M.M
        "#
        ),
        "1"
    );
}

#[test]
fn test_part2_simple_leftright() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part2(
            r#"
        S.M
        .A.
        S.M
        "#
        ),
        "1"
    );
}

#[test]
fn test_part2_simple_cross() {
    let day = Day4 {};
    assert_eq!(
        day.solve_part2(
            r#"
        M.S
        .A.
        S.M
        "#
        ),
        "0"
    );
}
