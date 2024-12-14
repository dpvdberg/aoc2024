use nalgebra::Vector2;
use once_cell::sync::Lazy;

pub static DIRECTION_VECTORS: Lazy<Vec<Vector2<i32>>> = Lazy::new(|| {
    vec![
        Vector2::new(1, 0),
        Vector2::new(-1, 0),
        Vector2::new(0, -1),
        Vector2::new(0, 1),
    ]
});
