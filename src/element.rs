pub mod button;
pub mod canvas;
pub mod icon;
pub mod text;

use std::rc::Rc;

use button::Button;
use canvas::Canvas;
use icon::Icon;
use slotmap::{new_key_type, SlotMap};

use crate::app::{Application, ElementId};
use crate::draw;
use crate::event::{Event, KeyEvent, MouseEvent, MouseMoveEvent};
use crate::react::SignalId;
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

    pub(crate) fn handle(&mut self, event: Event) {
        self.inner.update(&event);
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
            _ => self.get_bounding_box().intersects(target),
        }
    }
}

pub enum ElementInner {
    Icon(Icon),
    Canvas(Canvas),
    Button(Button),
}

impl ElementInner {
    fn draw(&self) -> (Vec<bool>, UVec2) {
        match self {
            ElementInner::Icon(ico) => ico.draw(),
            ElementInner::Canvas(cv) => cv.draw(),
            ElementInner::Button(but) => but.draw(),
        }
    }

    fn get_size(&self) -> UVec2 {
        use ElementInner as EI;

        match self {
            EI::Icon(ico) => ico.get_size(),
            EI::Canvas(cv) => cv.get_size(),
            EI::Button(but) => but.get_size(),
        }
    }

    fn update(&mut self, ev: &Event) {
        use ElementInner as EI;

        match self {
            EI::Button(but) => but.update(ev),
            _ => {}
        }
    }
}

// The key types stay private to avoid undefined behavior,
// since [`Key`](`slotmap::Key`) types can be crafted from unknown [`u64`]'s.
/// A reference to an event handler registered to an [`Element`] or [`Application`](`crate::app::Application`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerId {
    Mouse(MouseId),
    Key(KeyId),
    MouseMove(MouseMoveId),
}

new_key_type! {
    struct MouseId;
    struct MouseMoveId;
    struct KeyId;
}

struct Handlers {
    mouse_handlers: SlotMap<MouseId, Rc<dyn Fn(&mut Application, ElementId, MouseEvent)>>,
    mouse_move_handlers: SlotMap<MouseMoveId, Rc<dyn Fn(&mut Application, ElementId, MouseMoveEvent)>>,
    key_handlers: SlotMap<KeyId, Rc<dyn Fn(&mut Application, ElementId, KeyEvent)>>,
}

impl Handlers {

    fn remove_handler(&mut self, id: HandlerId) {
        match id {
            HandlerId::Mouse(id) => {
                self.mouse_handlers.remove(id);
            }
            HandlerId::MouseMove(id) => {
                self.mouse_move_handlers.remove(id);
            }
            HandlerId::Key(id) => {
                self.key_handlers.remove(id);
            }
        }
    }
}
