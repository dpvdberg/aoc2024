use crate::day23::Day23;
use crate::solution::Solution;

static SAMPLE: &str = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

#[test]
fn test_part1() {
    let day = Day23 {};
    assert_eq!(day.solve_part1(SAMPLE), "7");
}

#[test]
fn test_part2() {
    let day = Day23 {};
    assert_eq!(day.solve_part2(SAMPLE), "co,de,ka,ta");
}
