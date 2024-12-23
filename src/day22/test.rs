use crate::day22::{Buyer, Day22};
use crate::solution::Solution;

#[test]
fn test_part1() {
    let day = Day22 {};
    assert_eq!(day.solve_part1(r#"
    1
    10
    100
    2024
    "#), "37327623");
}

#[test]
fn test_part2() {
    let day = Day22 {};
    assert_eq!(day.solve_part2(r#"
    1
    2
    3
    2024
    "#), "23");
}

#[test]
fn test_ones() {
    let buyer = Buyer::new(123, 5);
    assert_eq!(*buyer.evolve_ones(), vec![3, 0, 6, 5, 4, 4]);
}

#[test]
fn test_changes() {
    let buyer = Buyer::new(123, 5);
    assert_eq!(*buyer.price_changes(), vec![-3, 6, -1, -1, 0]);
}
