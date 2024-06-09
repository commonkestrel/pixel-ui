pub mod draw;
pub mod element;
pub mod event;
pub mod react;
pub mod util;
pub mod app;

pub mod prelude {
    pub use super::app::{Application, ApplicationBuilder};
    pub use super::react::{WriteSignal, ReadSignal};
    pub use super::event::*;
    pub use super::element::{
        Element,
        canvas::Canvas,
        text::Text,
        icon::Icon,
        button::Button,
    };
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
