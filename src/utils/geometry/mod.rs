use nalgebra::{vector, Vector2};
use once_cell::sync::Lazy;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn to_vector(&self) -> Vector2<i32> {
        match self {
            Direction::Up => vector![0, -1],
            Direction::Down => vector![0, 1],
            Direction::Left => vector![-1, 0],
            Direction::Right => vector![1, 0],
        }
    }

    pub(crate) fn is_vertical(&self) -> bool {
        self == &Direction::Up || self == &Direction::Down
    }
}

pub static DIRECTION_VECTORS: Lazy<Vec<Vector2<i32>>> =
    Lazy::new(|| vec![vector![1, 0], vector![-1, 0], vector![0, -1], vector![0, 1]]);
