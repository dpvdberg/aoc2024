use crate::solution::Solution;
use nalgebra::{vector, Matrix2, Vector2};
use regex::Regex;

#[cfg(test)]
mod test;
pub struct Day13 {}

struct ClawMachine {
    button_a: Vector2<i64>,
    button_b: Vector2<i64>,
    price: Vector2<i64>,
}

impl ClawMachine {
    fn solve_button_count(&self, max: Option<i64>) -> Option<Vector2<i64>> {
        let matrix = Matrix2::new(
            self.button_a.x,
            self.button_b.x,
            self.button_a.y,
            self.button_b.y,
        );

        if let Some(inverse_matrix) = matrix.map(|v| v as f64).try_inverse() {
            let v_float = inverse_matrix * Vector2::new(self.price.x as f64, self.price.y as f64);
            let button_count : Vector2<i64> = Vector2::new(v_float.x.round() as i64, v_float.y.round() as i64);
            if let Some(m) = max {
                if button_count.x > m || button_count.y > m {
                    return None;
                }
            }
            
            if matrix * button_count == self.price {
                return Some(button_count);
            }
        }

        None
    }
    
    fn tokens_for_price(&self, max: Option<i64>) -> Option<Vector2<i64>> {
        if let Some(count) = self.solve_button_count(max) {
            return Some(Vector2::new(count.x * 3, count.y));
        }
        
        None
    }
}

static CLAW_DESCRIPTION_REGEX: &str =
    r"\Button A: X\+(\d+), Y\+(\d+)\r?\n\s*Button B: X\+(\d+), Y\+(\d+)\r?\n\s*Prize: X=(\d+), Y=(\d+)";
fn parse_input(input: &str) -> Vec<ClawMachine> {
    let re = Regex::new(CLAW_DESCRIPTION_REGEX).unwrap();

    re.captures_iter(input.trim())
        .map(|c| c.extract())
        .map(|(_, [a_x, a_y, b_x, b_y, p_x, p_y])| ClawMachine {
            button_a: vector![a_x.parse::<i64>().unwrap(), a_y.parse::<i64>().unwrap()],
            button_b: vector![b_x.parse::<i64>().unwrap(), b_y.parse::<i64>().unwrap()],
            price: vector![p_x.parse::<i64>().unwrap(), p_y.parse::<i64>().unwrap()],
        })
        .collect()
}

impl Solution for Day13 {
    fn solve_part1(input: &str) -> String {
        let machines = parse_input(input);
        
        machines.iter()
            .filter_map(|machine| machine.tokens_for_price(Some(100)))
            .map(|v| v.sum())
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(input: &str) -> String {
        let machines = parse_input(input);
        
        machines.iter()
            .map(|m| ClawMachine {
                button_a : m.button_a,
                button_b : m.button_b,
                price : m.price + vector![10000000000000, 10000000000000],
                
            })
            .filter_map(|machine| machine.tokens_for_price(None))
            .map(|v| v.sum())
            .sum::<i64>()
            .to_string()
    }
}
