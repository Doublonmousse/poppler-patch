use crate::Color;
use std::fmt;

impl Color {
    pub fn red(&self) -> u16 {
        self.inner.red
    }

    pub fn set_red(&mut self, red: u16) {
        self.inner.red = red;
    }

    pub fn green(&self) -> u16 {
        self.inner.green
    }

    pub fn set_green(&mut self, green: u16) {
        self.inner.green = green;
    }

    pub fn blue(&self) -> u16 {
        self.inner.blue
    }

    pub fn set_blue(&mut self, blue: u16) {
        self.inner.blue = blue;
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}
