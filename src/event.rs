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
    pub key: PhysicalKey,
    pub modifiers: Modifiers,
    pub state: ElementState,
    pub repeat: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub pos: IVec2,
    pub modifiers: Modifiers,
    pub state: ElementState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseMoveEvent {
    pub pos: IVec2,
    pub modifiers: Modifiers,
    pub delta: IVec2,
}

impl MouseMoveEvent {}
