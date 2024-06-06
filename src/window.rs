use std::{any::Any, cell::RefCell, rc::Rc};

use softbuffer::Surface;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
};

use crate::{
    element::Element,
    signal::{SignalId, SignalState},
    util::IVec2,
};

pub struct Window {
    mouse_position: IVec2,
    focused: Option<ElementId>,
    elements: Vec<Element>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            focused: None,
            mouse_position: IVec2::default(),
        }
    }

    pub fn run(&mut self) {
        
    }

    pub fn query_class<'a>(&'a self, target: &'a str) -> impl Iterator<Item = &'a Element> {
        self.elements.iter().filter(|el| el.contains_class(target))
    }

    pub fn insert_element<E: Into<Element>>(&mut self, el: E) {
        self.elements.push(el.into());
    }

    pub fn update(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                for el in self.elements.iter_mut() {
                    if el.intersects(self.mouse_position) {}
                    match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ElementId(usize);
