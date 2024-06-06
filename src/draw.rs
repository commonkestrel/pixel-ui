use crate::util::{BoundingBox, IVec2};

pub fn write(buf: &mut [bool], width: usize, height: usize, position: IVec2, input: bool) {
    if position.x >= 0 && position.y >= 0 {
        let x_pos = position.x as usize;
        let y_pos = position.y as usize;

        if x_pos < width && y_pos < height {
            buf[(x_pos + y_pos * width) as usize] = input;
        }
    }
}

pub fn write_all(
    buf: &mut [bool],
    width: usize,
    height: usize,
    bounds: BoundingBox,
    content: Vec<bool>,
) {
    // If either max is negative, we can just assume the whole contents are out of frame
    if bounds.get_max().x < 0 || bounds.get_max().y < 0 {
        return;
    }

    if bounds.area() != content.len() {
        return;
    }

    let left_blank = (-bounds.get_min().x).max(0);
    let top_blank = (-bounds.get_min().y).max(0);

    let max_x = (bounds.get_max().x as usize).min(width);
    let max_y = (bounds.get_max().y as usize).min(height);

    for x in 0..max_x {
        for y in 0..max_y {}
    }
}

pub struct Line {
    start: IVec2,
    end: IVec2,
}

impl Line {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        Self { start, end }
    }

    pub fn clamp(&mut self, bounds: BoundingBox) {
        let min = bounds.get_min();
        let max = bounds.get_max();

        self.start.x = self.start.x.min(max.x).max(min.x);
        self.start.y = self.start.x.min(max.y).max(min.y);
        self.end.x = self.end.x.min(max.x).max(min.x);
        self.end.y = self.end.y.min(max.y).max(min.y);
    }

    pub fn clamped(mut self, bounds: BoundingBox) -> Self {
        self.clamp(bounds);
        self
    }
}
