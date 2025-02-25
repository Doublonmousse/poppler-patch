use crate::Point;
use std::fmt;

impl Point {
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.inner.x = x;
    }

    pub fn y(&self) -> f64 {
        self.inner.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.inner.y = y;
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
