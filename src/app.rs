use std::{any::Any, cell::RefCell, rc::Rc};

use slotmap::{new_key_type, SlotMap};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::{ElementState, Modifiers, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowAttributes
};

use crate::{
    element::Element, react::{Context, ProxyEvent, SignalId, SignalState}, util::{IVec2, UVec2}
};

new_key_type! { pub struct ElementId; }

pub struct ApplicationBuilder {
    event_loop: EventLoop<ProxyEvent>,
    elements: SlotMap<ElementId, Element>,
    context: Context,
    window_attributes: Option<WindowAttributes>,
}

impl ApplicationBuilder {
    pub fn run(self) -> Result<(), EventLoopError> {
        let mut app = Application {
            window: None,
            window_attributes: self.window_attributes.unwrap_or_default(),
            mouse_position: IVec2::default(),
            modifiers: Modifiers::default(),
            focused: None,
            elements: self.elements,
            context: self.context,
        };

        self.event_loop.run_app(&mut app)?;

        Ok(())
    }

    pub fn with_attributes(mut self, attrs: WindowAttributes) -> Self {
        self.set_attributes(attrs);
        self
    }

    pub fn set_attributes(&mut self, attrs: WindowAttributes) {
        self.window_attributes = Some(attrs);
    }

    pub fn query_class<'a>(&'a self, target: &'a str) -> impl Iterator<Item = &'a Element> {
        self.elements.values().filter(|el| el.contains_class(target))
    }

    pub fn insert_element<E: Into<Element>>(&mut self, el: E) -> ElementId {
        self.elements.insert(el.into())
    }
}

pub struct Application {
    window: Option<winit::window::Window>,
    window_attributes: WindowAttributes,
    mouse_position: IVec2,
    modifiers: Modifiers,
    focused: Option<ElementId>,
    elements: SlotMap<ElementId, Element>,
    context: Context,
}

impl Application {
    pub fn builder() -> Result<ApplicationBuilder, EventLoopError> {
        let event_loop = EventLoop::with_user_event().build()?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let proxy = event_loop.create_proxy();

        Ok(ApplicationBuilder {
            event_loop,
            elements: SlotMap::with_key(),
            context: Context::new(proxy),
            window_attributes: None,
        })
    }

    pub fn insert_element<E: Into<Element>>(&mut self, element: E) -> ElementId {
        self.elements.insert(element.into())
    }

    pub fn remove_element(&mut self, id: ElementId) -> Option<Element> {
        self.elements.remove(id)
    }

    pub fn get(&self, id: ElementId) -> &Element {
        self.elements.get(id).expect("element should exist")
    }

    pub fn try_get(&self, id: ElementId) -> Option<&Element> {
        self.elements.get(id)
    }

    pub fn get_mut(&mut self, id: ElementId) -> &mut Element {
        self.elements.get_mut(id).expect("element should exist")
    }

    pub fn try_get_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.get_mut(id)
    }
}

impl ApplicationHandler<ProxyEvent> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(self.window_attributes.clone()).expect("should be able to create window"))
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = None;
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::MouseInput {
                device_id: _,
                state, button,
            } => {
                
            }
            _ => {}
        }
    }
}
