use winit::{
    event::{ElementState, Modifiers},
    keyboard::PhysicalKey,
};

use crate::util::{IVec2, UVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Mouse(MouseEvent),
    Key(KeyEvent),
    MouseMove(MouseMoveEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    key: PhysicalKey,
    modifiers: Modifiers,
    state: ElementState,
    repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pos: IVec2,
    modifiers: Modifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseMoveEvent {
    pos: IVec2,
    modifiers: Modifiers,
    delta: IVec2,
}

impl MouseMoveEvent {}
