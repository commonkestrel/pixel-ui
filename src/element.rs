pub mod body;
pub mod button;
pub mod canvas;
pub mod icon;
pub mod span;
pub mod text;

use std::rc::Rc;

use body::Body;
use icon::Icon;

use crate::draw;
use crate::event::{Event, KeyEvent, MouseEvent, MouseMoveEvent};
use crate::signal::SignalId;
use crate::util::{BoundingBox, IVec2, UVec2};

pub struct Element {
    inner: ElementInner,
    classes: Vec<String>,
    hidden: bool,
    offset: IVec2,
    z_index: usize,
    handlers: Handlers,
    subscriptions: Vec<SignalId>,
}

impl Element {
    pub fn append_class(&mut self, class: String) {
        self.classes.push(class);
    }

    pub fn get_classes(&self) -> &[String] {
        &self.classes
    }

    pub fn get_z_index(&self) -> usize {
        self.z_index
    }

    pub fn set_z_index(&mut self, z_index: usize) {
        self.z_index = z_index;
    }

    fn draw(&self, buf: &mut [bool], width: usize, height: usize) {
        if !self.hidden {
            let (graphic, size) = self.inner.draw();

            draw::write_all(
                buf,
                width,
                height,
                BoundingBox::from_size(self.offset, size),
                graphic,
            );
        }
    }

    pub(crate) fn update(&mut self, event: Event) {
        self.inner.update(&event);
        match event {
            Event::Key(ev) => self
                .handlers
                .key_handlers
                .clone()
                .into_iter()
                .for_each(|handler| handler(self, ev)),
            Event::Mouse(ev) => self
                .handlers
                .mouse_handlers
                .clone()
                .into_iter()
                .for_each(|handler| handler(self, ev)),
            Event::MouseMove(ev) => self
                .handlers
                .mouse_move_handlers
                .clone()
                .into_iter()
                .for_each(|handler| handler(self, ev)),
        }
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn set_offset(&mut self, offset: IVec2) {
        self.offset = offset;
    }

    pub fn with_offset(mut self, offset: IVec2) -> Self {
        self.set_offset(offset);
        self
    }

    pub fn contains_class(&self, target: &str) -> bool {
        for class in self.get_classes() {
            if class == target {
                return true;
            }
        }

        return false;
    }

    pub fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_size(self.offset, self.inner.get_size())
    }

    pub fn remove_handler(&mut self, id: HandlerId) {
        self.handlers.remove_handler(id);
    }

    pub(crate) fn subscribed(&self, signal: &SignalId) -> bool {
        self.subscriptions.contains(signal)
    }

    pub fn intersects(&self, target: IVec2) -> bool {
        match self.inner {
            ElementInner::Body(_) => true,
            _ => self.get_bounding_box().intersects(target),
        }
    }
}

pub enum ElementInner {
    Body(Body),
    Icon(Icon),
}

impl ElementInner {
    fn draw(&self) -> (Vec<bool>, UVec2) {
        match self {
            ElementInner::Body(_body) => (Vec::new(), UVec2::default()),
            ElementInner::Icon(icon) => icon.draw(),
        }
    }

    fn get_size(&self) -> UVec2 {
        use ElementInner as EI;

        match self {
            EI::Body(_) => UVec2::default(),
            EI::Icon(icon) => icon.get_size(),
        }
    }

    fn update(&mut self, ev: &Event) {
        use ElementInner as EI;

        match self {
            EI::Body(_) => {}
            EI::Icon(_) => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandlerId {
    ty: CallbackType,
    idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallbackType {
    Click,
    Key,
    MouseMove,
}

struct Handlers {
    mouse_handlers: Vec<Rc<dyn Fn(&mut Element, MouseEvent)>>,
    mouse_move_handlers: Vec<Rc<dyn Fn(&mut Element, MouseMoveEvent)>>,
    key_handlers: Vec<Rc<dyn Fn(&mut Element, KeyEvent)>>,
}

impl Handlers {
    fn handle(&self, event: Event) {}

    fn remove_handler(&mut self, id: HandlerId) {
        match id.ty {
            CallbackType::Click => {
                self.mouse_handlers.remove(id.idx);
            }
            CallbackType::MouseMove => {
                self.mouse_move_handlers.remove(id.idx);
            }
            CallbackType::Key => {
                self.key_handlers.remove(id.idx);
            }
        }
    }
}
