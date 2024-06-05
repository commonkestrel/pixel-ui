use winit::{event::{ElementState, Modifiers}, keyboard::PhysicalKey};

use crate::util::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    MouseDown(Vec2),
    KeyEvent(KeyEvent),
    MouseMove {
        start: Vec2,
        end: Vec2,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    key: PhysicalKey,
    modifiers: Modifiers,
    state: ElementState,
    repeat: bool,
}
