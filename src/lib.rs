pub mod draw;
pub mod element;
pub mod event;
pub mod react;
pub mod color;
pub mod app;
mod util;

pub mod prelude {
    pub use super::app::{Application, ApplicationBuilder};
    pub use super::react::{WriteSignal, ReadSignal};
    pub use super::color::Color;
    pub use super::event::*;
    pub use super::element::{
        Element,
        canvas::Canvas,
        text::Text,
        icon::Icon,
        button::Button,
        rect::Rect,
    };
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
