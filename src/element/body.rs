use crate::{component::Component, util::Position};

pub struct Body<E=()> {
    click_callbacks: Vec<fn (pos: Position) -> E>
}

impl Component for Body {
    fn set_hidden(&mut self, hidden: bool) {}
    fn set_offset(&mut self, offset: crate::util::Position) {}
    fn
}