use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn round(self) -> Self {
        self.x.round();
        self.y.round();
        self
    }

    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .partial_cmp(&other.x)
            .and_then(|ord| {
                if ord == Ordering::Equal {
                    self.y.partial_cmp(&other.y)
                } else {
                    Some(ord)
                }
            }).expect("Could not compare values")
    }
}

impl Eq for Point {}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point { x, y }
    }
}
