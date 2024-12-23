use nalgebra::{vector, DMatrix, Matrix, Scalar, Storage, Vector2};

pub trait MatrixParser<T> {
    fn to_matrix(&self, char_map: fn(char) -> T) -> DMatrix<T>;
}

impl<T: PartialEq + Clone + std::fmt::Debug + 'static> MatrixParser<T> for String {
    fn to_matrix(&self, char_map: fn(char) -> T) -> DMatrix<T> {
        let elements: Vec<Vec<T>> = self
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| l.chars().map(|c| char_map(c)).collect())
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

pub trait VectorHelpers {
    fn to_matrix_index(&self) -> (usize, usize);

    fn get_intermediate_points(&self, end: &Vector2<i32>) -> Vec<Vector2<i32>>;
}

impl VectorHelpers for Vector2<i32> {
    fn to_matrix_index(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }

    fn get_intermediate_points(&self, end: &Vector2<i32>) -> Vec<Vector2<i32>> {
        let mut points = Vec::new();

        let start = self;

        let mut current_x = start.x;
        let mut current_y = start.y;

        let delta_x = (end.x - start.x).abs();
        let delta_y = -(end.y - start.y).abs();

        let step_x = if start.x < end.x { 1 } else { -1 };
        let step_y = if start.y < end.y { 1 } else { -1 };

        let mut error_term = delta_x + delta_y;

        loop {
            points.push(vector![current_x, current_y]);

            if current_x == end.x && current_y == end.y {
                break;
            }

            let double_error = 2 * error_term;
            if double_error >= delta_y {
                error_term += delta_y;
                current_x += step_x;
            }
            if double_error <= delta_x {
                error_term += delta_x;
                current_y += step_y;
            }
        }

        points
    }
}

pub trait MatrixHelpers<T> {
    fn at(&self, location: &Vector2<i32>) -> &T;
    fn at_value(&self, location: &Vector2<i32>) -> T;
    fn valid_index(&self, l: &Vector2<i32>) -> bool;
    fn find_index(&self, element: T) -> Option<Vector2<i32>>;
}

impl<T, R, C, S> MatrixHelpers<T> for Matrix<T, R, C, S>
where
    T: Scalar + PartialEq + Clone,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: Storage<T, R, C>,
{
    fn at(&self, location: &Vector2<i32>) -> &T {
        &self[location.to_matrix_index()]
    }

    fn at_value(&self, location: &Vector2<i32>) -> T {
        self[location.to_matrix_index()].clone()
    }

    fn valid_index(&self, l: &Vector2<i32>) -> bool {
        l.x >= 0 && l.x < self.ncols() as i32 && l.y >= 0 && l.y < self.nrows() as i32
    }

    fn find_index(&self, element: T) -> Option<Vector2<i32>> {
        (0..self.ncols())
            .flat_map(|x| (0..self.nrows()).map(move |y| vector![x as i32, y as i32]))
            .find(|v: &Vector2<i32>| *self.at(v) == element)
    }
}
