use crate::util::UVec2;

pub struct Span {
    size: UVec2,
    color: bool
}

impl Span {
    pub fn new(size: UVec2, color: bool) -> Self {
        Self {size, color}
    }
}
