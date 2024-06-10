use winit::event::{ElementState, MouseButton};

use crate::{color::Color, event::Event, util::UVec2};

pub struct Button {
    size: UVec2,
    depressed: bool,
    color: Color,
}

impl Button {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self {
            size: UVec2::new(width, height),
            depressed: false,
            color,
        }
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }

    pub(crate) fn draw(&self) -> (Vec<Color>, UVec2) {
        // If the size is less than or equal to 4, the content will just be
        // the border, so we can just fill the content with the border color
        if self.size.x <= 4 || self.size.y <= 4 {
            let color = if self.depressed { self.color } else { !self.color };
            let buf = vec![color; self.size.area()];

            return (buf, self.size);
        }

        let mut buf = vec![self.color; self.size.area()];

        if self.depressed {
            buf.iter_mut().for_each(|pixel| *pixel = !*pixel);
        }

        for y in 0..self.size.y {
            let y_offset = y*self.size.x;

            // left border
            buf[y_offset] = !self.color;
            buf[y_offset + 1] = !self.color;

            // right border
            buf[y_offset + self.size.x - 1] = !self.color;
            buf[y_offset + self.size.x - 2] = !self.color;
        }

        for x in 0..self.size.x {
            // top border
            buf[x] = !self.color;
            buf[x + self.size.x] = !self.color;

            // bottom border
            buf[x + (self.size.y-1)*self.size.x] = !self.color;
            buf[x + (self.size.y-2)*self.size.x] = !self.color;
        }

        (buf, self.size)
    }

    pub fn update(&mut self, ev: &Event) {
        match ev {
            // Update button depressed state
            Event::Mouse(event) => {
                if event.button == MouseButton::Left {
                    self.depressed = event.state == ElementState::Pressed
                }
            },
            _ => {}
        }
    }
}
