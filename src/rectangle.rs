use crate::Rectangle;
use std::fmt;

impl Rectangle {
    pub fn x1(&self) -> f64 {
        self.inner.x1
    }

    pub fn set_x1(&mut self, x1: f64) {
        self.inner.x1 = x1;
    }

    pub fn y1(&self) -> f64 {
        self.inner.y1
    }

    pub fn set_y1(&mut self, y1: f64) {
        self.inner.y1 = y1;
    }

    pub fn x2(&self) -> f64 {
        self.inner.x2
    }

    pub fn set_x2(&mut self, x2: f64) {
        self.inner.x2 = x2;
    }

    pub fn y2(&self) -> f64 {
        self.inner.y2
    }

    pub fn set_y2(&mut self, y2: f64) {
        self.inner.y2 = y2;
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x1", &self.x1())
            .field("y1", &self.y1())
            .field("x2", &self.x2())
            .field("y2", &self.y2())
            .finish()
    }
}
