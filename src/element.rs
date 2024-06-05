pub mod body;

use body::Body;

use crate::{component::Component, util::{BoundingBox, Vec2}};

pub struct Element<E> {
    inner: ElementInner,
    classes: Vec<String>,
    hidden: bool,
    offset: Vec2,
}

impl<E> Element<E> {
    fn append_class(&mut self, class: String) {
        self.classes.push(class);
    }
    
    fn get_classes(&self) -> &[String] {
        &self.classes
    }

    fn draw(&self, buf: &mut [u8], width: usize, height: usize) {
        if !self.props.hidden {
            let el = self.inner.draw();
        }
    }

    pub(crate) fn update(&mut self, events: Vec<Event>);
    pub(crate) fn draw(&self, buf: &mut [u8], width: usize, height: usize);
    
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
    
    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.set_offset(offset);
        self
    }

    fn contains_class(&self, target: &str) -> bool {
        for class in self.get_classes() {
            if class == target {
                return true;
            }
        }

        return false
    }

    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_size(self.offset, self.inner.get_size())
    }

    fn on_click(&mut self, callback: fn (pos: Vec2) -> E) -> CallbackId {
        self.click_callbacks.push(callback);
    }

    fn on_key_event(&mut self, ev: KeyEvent) -> CallbackId;
    fn remove_callback(&mut self, id: CallbackId) {
        match id.ty {
            CallbackType::Click => self.click_callbacks.remove(id.idx),
        }
    }
}

pub enum ElementInner {
    Body(Body),
}

impl ElementInner {
    fn draw(&self) -> Vec<Vec<bool>> {

    }
}

pub struct Properties {
    hidden: bool,
    offset: Vec2,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            hidden: false,
            offset: Vec2::default(),
        }
    }
}

pub struct CallbackId {
    ty: CallbackType,
    idx: usize,
}

enum CallbackType {
    Click,
    KeyEvent,
}
