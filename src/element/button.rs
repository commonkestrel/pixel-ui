use winit::event::ElementState;

use crate::{event::Event, util::UVec2};

pub struct Button {
    size: UVec2,
    border_thickness: usize,
    depressed: bool,
}

impl Button {
    const DEFAULT_BORDER_THICKNESS: usize = 2;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: UVec2::new(width, height),
            border_thickness: Self::DEFAULT_BORDER_THICKNESS,
            depressed: false,
        }
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }

    pub(crate) fn draw(&self) -> (Vec<bool>, UVec2) {
        (Vec::new(), UVec2::default())
    }

    pub fn update(&mut self, ev: &Event) {
        match ev {
            // set depressed state to whether
            Event::Mouse(event) => self.depressed = event.state == ElementState::Pressed,
            _ => {}
        }
    }
}
