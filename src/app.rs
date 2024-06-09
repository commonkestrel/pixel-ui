use std::{any::Any, cell::RefCell, num::{NonZeroU32, NonZeroUsize}, rc::Rc};

use slotmap::{new_key_type, SlotMap};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::{ElementState, Modifiers, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowAttributes}
};

use crate::{
    element::Element, event::{Event, MouseEvent}, react::{Context, ProxyEvent, SignalId, SignalState}, util::{IVec2, UVec2}
};

new_key_type! { pub struct ElementId; }

pub struct ApplicationBuilder {
    event_loop: EventLoop<ProxyEvent>,
    elements: SlotMap<ElementId, Element>,
    context: Context,
    window_attributes: Option<WindowAttributes>,
    background: bool,
}

impl ApplicationBuilder {
    pub fn run(self) -> Result<(), EventLoopError> {
        let mut app = Application {
            surface: None,
            window_attributes: self.window_attributes.unwrap_or_default(),
            background: self.background,
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

    pub fn with_background(mut self, background: bool) -> Self {
        self.set_background(background);
        self
    }

    pub fn set_background(&mut self, background: bool) {
        self.background = background;
    }

    pub fn query_class<'a>(&'a self, target: &'a str) -> impl Iterator<Item = &'a Element> {
        self.elements.values().filter(|el| el.contains_class(target))
    }

    pub fn insert_element<E: Into<Element>>(&mut self, el: E) -> ElementId {
        self.elements.insert(el.into())
    }
}

pub struct Application {
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    window_attributes: WindowAttributes,
    background: bool,
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
            background: false,
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

    pub fn get_background(&self) -> bool {
        self.background
    }

    pub fn set_background(&mut self, background: bool) {
        self.background = background;
    }
}

impl ApplicationHandler<ProxyEvent> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Rc::new(event_loop.create_window(self.window_attributes.clone()).expect("should be able to create window"));

        let context = softbuffer::Context::new(window.clone()).expect("should be able to create draw context");
        let surface = Surface::new(&context, window).expect("should be able to create draw surface");

        self.surface = Some(surface);
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.surface = None;
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
                let ev = MouseEvent {
                    pos: self.mouse_position,
                    modifiers: self.modifiers,
                    state,
                    button,
                };

                let keys: Vec<ElementId> = self.elements.keys().collect();
                for key in keys {
                    if let Some(el) = self.elements.get(key) {
                        let handlers: Vec<Rc<dyn Fn(&mut Application, ElementId, MouseEvent)>> = el.handlers.mouse_handlers.values().cloned().collect();
                        for handler in handlers {
                            handler(self, key, ev);
                        }
                    }
                }

                self.surface.as_ref().expect("window should exist").window().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_ref().expect("draw surface should exist");
                let window = surface.window();

                let dimensions = window.inner_size();
                let size = UVec2::new(dimensions.width as usize, dimensions.height as usize);

                let mut buffer = vec![self.background; size.area()];

                for element in self.elements.values() {
                    element.draw(&mut buffer, size.x, size.y);
                }

                window.pre_present_notify();

                let surface = self.surface.as_mut().expect("draw surface should exist");
                surface.resize(
                    NonZeroU32::new(dimensions.width).expect("window width should be greater than 0"),
                    NonZeroU32::new(dimensions.height).expect("window height should be greater than 0")
                ).expect("should be able to resize draw buffer");

                let mut window_buffer = surface.buffer_mut().expect("should be able to retrieve draw buffer");
                window_buffer.iter_mut().zip(buffer.into_iter()).for_each(|(current, write)| {
                    *current = if write { 0x00FFFFFF } else { 0x0000 };
                });

                window_buffer.present().expect("should be able to present buffer");
            }
            _ => {}
        }
    }
}
