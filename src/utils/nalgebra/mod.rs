use nalgebra::{DMatrix, Vector2};

pub trait MatrixIndex {
    fn to_matrix_index(&self) -> (usize, usize);
}

impl MatrixIndex for Vector2<i32> {
    fn to_matrix_index(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}


pub trait MatrixAt<T> {
    fn at(&self, location: &Vector2<i32>) -> &T;
}

impl<T> MatrixAt<T> for DMatrix<T> {
    fn at(&self, location: &Vector2<i32>) -> &T {
        &self[location.to_matrix_index()]
    }
}