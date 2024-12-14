use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T, U> From<(U, U)> for Point<T>
where
    Point<T>: From<Point<U>>,
    T: From<U>,
{
    fn from(value: (U, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl From<Point<i32>> for Point<f32> {
    fn from(value: Point<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl<T, U> ops::Add<U> for Point<T>
where
    Point<T>: From<U>,
    T: ops::Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: U) -> Self::Output {
        let other: Point<T> = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T, U> ops::Sub<U> for Point<T>
where
    Point<T>: From<U>,
    T: ops::Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, other: U) -> Self::Output {
        let other: Point<T> = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
