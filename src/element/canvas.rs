use std::{cell::RefCell, rc::Rc};

use crate::util::{BoundingBox, UVec2};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    size: UVec2,
    content: Rc<RefCell<Vec<u8>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawContext {
    size: UVec2,
    content: Rc<RefCell<Vec<u8>>>,
}

impl DrawContext {
    pub fn pixel(&self, position: UVec2, value: bool) {
        if position.x >= self.size.x || position.y >= self.size.y {
            return;
        }

        let pixel = position.y * self.size.x + position.x;
        let index = pixel / 8;
        let bit = pixel % 8;

        if value {
            self.content.borrow_mut()[index] |= 1 << bit;
        } else {
            self.content.borrow_mut()[index] &= !(1 << bit);
        };
    }

    pub fn rect(&self, bounds: BoundingBox, value: bool) {
        let x1 = bounds.get_min().x.max(0) as usize;
        let y1 = bounds.get_min().y.max(0) as usize;
        let x2 = (bounds.get_max().x.max(0) as usize).min(self.size.x-1);
        let y2 = (bounds.get_max().y.max(0) as usize).min(self.size.y-1);

        let mut content = self.content.borrow_mut();

        for y in y1..=y2 {
            let y_offset = y * self.size.x;
            for x in x1..=x2 {
                let pixel = y_offset + x;
                let index = pixel / 8;
                let bit = pixel % 8;

                if value {
                    content[index] |= 1 << bit;
                } else {
                    content[index] &= !(1 << bit);
                };
            }
        }
    }

    pub fn fill(&self, value: bool) {
        self.content.borrow_mut().fill(if value { 0xFF } else { 0x00 });
    }
}
