use crate::{color::Color, util::UVec2};

pub struct Rect {
    size: UVec2,
    color: Color,
}

impl Rect {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self {
            size: UVec2::new(width, height),
            color,
        }
    }

    pub fn draw(&self) -> (Vec<Color>, UVec2) {
        let buf = vec![self.color; self.size.area()];
        (buf, self.size)
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }
}
