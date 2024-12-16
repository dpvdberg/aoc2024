use nalgebra::{DMatrix, Vector2};

pub trait MatrixParser<T> {
    fn to_matrix(&self, char_map: fn(char) -> T) -> DMatrix<T>;
}

impl<T: PartialEq + Clone + std::fmt::Debug + 'static> MatrixParser<T> for String {
    fn to_matrix(&self, char_map: fn(char) -> T) -> DMatrix<T> {
        let elements: Vec<Vec<T>> = self
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                l.chars()
                    .map(|c| char_map(c))
                    .collect()
            })
            .collect();

        let rows = elements.len();
        let columns = elements.first().map_or(0, |l| l.len());

        if elements.iter().any(|l| l.len() != columns) {
            panic!("Not all lines have the same length.")
        }

        let flattened = elements
            .iter()
            .flatten()
            .map(|c| c.clone())
            .collect::<Vec<T>>();

        DMatrix::from_row_iterator(rows, columns, flattened)
    }
}

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