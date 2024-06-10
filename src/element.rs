pub mod button;
pub mod canvas;
pub mod icon;
pub mod text;
pub mod rect;

use std::rc::Rc;
use button::Button;
use canvas::Canvas;
use icon::Icon;
use rect::Rect;
use slotmap::{new_key_type, SlotMap};

use crate::app::{Application, ElementId, KeyId, MouseId, MouseMoveId, RehydrateId, ResizeId};
use crate::draw;
use crate::color::Color;
use crate::event::{Event, KeyEvent, MouseEvent, MouseMoveEvent};
use crate::prelude::ResizeEvent;
use crate::util::{BoundingBox, IVec2, UVec2};

pub struct Element {
    inner: ElementInner,
    classes: Vec<String>,
    hidden: bool,
    offset: IVec2,
    z_index: usize,
    pub(crate) handlers: Handlers,
}

impl Element {
    pub fn new<E: Into<ElementInner>>(element: E) -> Self {
        Self {
            inner: element.into(),
            classes: Vec::new(),
            hidden: false,
            offset: IVec2::default(),
            z_index: 0,
            handlers: Handlers::default(),
        }
    }

    pub fn button(width: usize, height: usize, color: Color) -> Element {
        Element::new(Button::new(width, height, color))
    }

    pub fn rect(width: usize, height: usize, color: Color) -> Element {
        Element::new(Rect::new(width, height, color))
    }

    pub fn append_class(&mut self, class: String) {
        self.classes.push(class);
    }

    pub fn with_class(mut self, class: String) -> Self {
        self.append_class(class);
        self
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

    pub(crate) fn draw(&self, buf: &mut [Color], width: usize, height: usize) {
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
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn set_offset(&mut self, x: isize, y: isize) {
        self.offset = IVec2::new(x, y);
    }

    pub fn with_offset(mut self, x: isize, y: isize) -> Self {
        self.set_offset(x, y);
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

    pub fn intersects(&self, target: IVec2) -> bool {
        match self.inner {
            _ => self.get_bounding_box().intersects(target),
        }
    }

    pub fn on_click(&mut self, f: impl Fn(&mut Application, ElementId, MouseEvent) + 'static) -> HandlerId {
        let id = self.handlers.mouse_handlers.insert(Rc::new(f));
        HandlerId::Mouse(id)
    }

    pub fn on_rehydrate(&mut self, f: impl Fn(&mut Application, ElementId) + 'static) -> HandlerId {
        let id = self.handlers.rehydrate_handlers.insert(Rc::new(f));
        HandlerId::Rehydrate(id)
    }

    pub fn on_resize(&mut self, f: impl Fn(&mut Application, ElementId, ResizeEvent) + 'static) -> HandlerId {
        let id = self.handlers.resize_handlers.insert(Rc::new(f));
        HandlerId::Resize(id)
    }
}
impl From<Canvas> for Element {
    fn from(value: Canvas) -> Self {
        Element::new(value)
    }
}
impl From<Icon> for Element {
    fn from(value: Icon) -> Self {
        Element::new(value)
    }
}

impl From<Button> for Element {
    fn from(value: Button) -> Self {
        Element::new(value)
    }
}

pub enum ElementInner {
    Icon(Icon),
    Canvas(Canvas),
    Button(Button),
    Rect(Rect),
}

impl ElementInner {
    fn draw(&self) -> (Vec<Color>, UVec2) {
        match self { 
            ElementInner::Icon(ico) => ico.draw(),
            ElementInner::Canvas(cv) => cv.draw(),
            ElementInner::Button(but) => but.draw(),
            ElementInner::Rect(rec) => rec.draw(),
        }
    }

    fn get_size(&self) -> UVec2 {
        use ElementInner as EI;

        match self {
            EI::Icon(ico) => ico.get_size(),
            EI::Canvas(cv) => cv.get_size(),
            EI::Button(but) => but.get_size(),
            EI::Rect(rec) => rec.get_size(),
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

impl From<Canvas> for ElementInner {
    fn from(value: Canvas) -> Self {
        ElementInner::Canvas(value)
    }
}

impl From<Icon> for ElementInner {
    fn from(value: Icon) -> Self {
        ElementInner::Icon(value)
    }
}

impl From<Button> for ElementInner {
    fn from(value: Button) -> Self {
        ElementInner::Button(value)
    }
}

impl From<Rect> for ElementInner {
    fn from(value: Rect) -> Self {
        ElementInner::Rect(value)
    }
}

// The key types stay private to avoid undefined behavior,
// since [`Key`](`slotmap::Key`) types can be crafted from unknown [`u64`]'s.
#[allow(private_interfaces)]
/// A reference to an event handler registered to an [`Element`] or [`Application`](`crate::app::Application`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerId {
    Mouse(MouseId),
    Key(KeyId),
    MouseMove(MouseMoveId),
    Rehydrate(RehydrateId),
    Resize(ResizeId)
}

pub(crate) struct Handlers {
    pub(crate) mouse_handlers: SlotMap<MouseId, Rc<dyn Fn(&mut Application, ElementId, MouseEvent)>>,
    pub(crate) mouse_move_handlers: SlotMap<MouseMoveId, Rc<dyn Fn(&mut Application, ElementId, MouseMoveEvent)>>,
    pub(crate) key_handlers: SlotMap<KeyId, Rc<dyn Fn(&mut Application, ElementId, KeyEvent)>>,
    pub(crate) rehydrate_handlers: SlotMap<RehydrateId, Rc<dyn Fn(&mut Application, ElementId)>>,
    pub(crate) resize_handlers: SlotMap<ResizeId, Rc<dyn Fn(&mut Application, ElementId, ResizeEvent)>>,
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
            HandlerId::Rehydrate(id) => {
                self.rehydrate_handlers.remove(id);
            }
            HandlerId::Resize(id) => {
                self.resize_handlers.remove(id);
            }
        }
    }
}

impl Default for Handlers {
    fn default() -> Self {
        Self {
            mouse_handlers: SlotMap::with_key(),
            mouse_move_handlers: SlotMap::with_key(),
            key_handlers: SlotMap::with_key(),
            rehydrate_handlers: SlotMap::with_key(),
            resize_handlers: SlotMap::with_key(),
        }
    }
}
