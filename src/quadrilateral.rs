use crate::{ffi, Point, Quadrilateral};
use std::fmt;

impl Quadrilateral {
    pub fn p1(&self) -> &Point {
        unsafe { &*(&self.inner.p1 as *const ffi::PopplerPoint as *const Point) }
    }

    pub fn set_p1(&mut self, p1: Point) {
        self.inner.p1 = p1.inner;
    }

    pub fn p2(&self) -> &Point {
        unsafe { &*(&self.inner.p2 as *const ffi::PopplerPoint as *const Point) }
    }

    pub fn set_p2(&mut self, p2: Point) {
        self.inner.p2 = p2.inner;
    }

    pub fn p3(&self) -> &Point {
        unsafe { &*(&self.inner.p3 as *const ffi::PopplerPoint as *const Point) }
    }

    pub fn set_p3(&mut self, p3: Point) {
        self.inner.p3 = p3.inner;
    }

    pub fn p4(&self) -> &Point {
        unsafe { &*(&self.inner.p4 as *const ffi::PopplerPoint as *const Point) }
    }

    pub fn set_p4(&mut self, p4: Point) {
        self.inner.p4 = p4.inner;
    }
}

impl fmt::Debug for Quadrilateral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Quadrilateral")
            .field("p1", &self.p1())
            .field("p2", &self.p2())
            .field("p3", &self.p3())
            .field("p4", &self.p4())
            .finish()
    }
}
