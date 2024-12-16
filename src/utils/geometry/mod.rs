use nalgebra::{vector, Vector2};
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Hash, Debug, EnumIter)]
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
    
    pub(crate) fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub(crate) fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub(crate) fn is_vertical(&self) -> bool {
        self == &Direction::Up || self == &Direction::Down
    }
}
