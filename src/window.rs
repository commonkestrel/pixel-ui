use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::{component::Component, element::Element};

pub struct Window<E=()> {
    win: winit::window::Window,
    elements: Vec<Element>,
    selected: Option<usize>,
}

impl<E> Window<E> {
    pub fn new(win: winit::window::Window) -> Self {
        Self {
            win,
            elements: Vec::new(),
            selected: None,
        }
    }

    pub fn query_class(&self, target: &str) -> () {
        self.elements.iter().filter(|el| el.contains_class(target))
    }

    pub fn respond(&mut self, event: WindowEvent) -> Vec<E> {
        match event {
            WindowEvent::MouseInput { device_id, state, button } => {
                for el in self.elements.iter_mut() {
                    
                }
            }
            _ => {}
        }
    }
}
