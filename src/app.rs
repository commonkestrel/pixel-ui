use std::{any::Any, cell::RefCell, num::{NonZeroU32, NonZeroUsize}, rc::Rc};

use chrono::TimeDelta;
use slotmap::{new_key_type, SlotMap};
use softbuffer::Surface;
use winit::{
    application::ApplicationHandler, error::EventLoopError, event::{ElementState, Modifiers, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowAttributes}
};

use crate::{
    color::Color, element::Element, event::{Event, MouseEvent}, prelude::{MouseMoveEvent, ReadSignal, ResizeEvent, WriteSignal}, react::{Context, Ctx, IntervalId, ProxyEvent, SignalId, TimeoutId}, util::{IVec2, UVec2}
};

new_key_type! { 
    pub struct ElementId;

    pub(crate) struct ResizeId;
    pub(crate) struct MouseId;
    pub(crate) struct MouseMoveId;
    pub(crate) struct KeyId;
    pub(crate) struct RehydrateId;
}

pub enum AppHandlerId {
    Resize(ResizeId),
}

pub struct ApplicationBuilder {
    event_loop: EventLoop<ProxyEvent>,
    elements: SlotMap<ElementId, Element>,
    context: Context,
    window_attributes: Option<WindowAttributes>,
    background: Color,

    resize_handlers: SlotMap<ResizeId, Rc<dyn Fn(&mut Application, ResizeEvent)>>,
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
            ctx: self.context,

            resize_handlers: SlotMap::with_key(),
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

    pub fn with_background(mut self, background: Color) -> Self {
        self.set_background(background);
        self
    }

    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    pub fn query_class<'a>(&'a self, target: &'a str) -> impl Iterator<Item = &'a Element> {
        self.elements.values().filter(|el| el.contains_class(target))
    }

    pub fn insert_element<E: Into<Element>>(&mut self, el: E) -> ElementId {
        self.elements.insert(el.into())
    }

    pub fn create_signal<T: 'static>(&mut self, init: T) -> (ReadSignal<T>, WriteSignal<T>) {
        self.context.create_signal(init)
    }

    pub fn set_timeout(&mut self, delay: TimeDelta, f: impl FnOnce(&mut Application) + 'static) -> TimeoutId {
        self.context.set_timeout(delay, f)
    }

    pub fn clear_timeout(&mut self, id: TimeoutId) {
        self.context.clear_timeout(id);
    }

    pub fn set_interval(&mut self, delay: TimeDelta, f: impl Fn(&mut Application) + 'static) -> IntervalId {
        self.context.set_interval(delay, f)
    }

    pub fn clear_interval(&mut self, id: IntervalId) {
        self.context.clear_interval(id);
    }

    pub fn on_resize(&mut self, f: impl Fn(&mut Application, ResizeEvent) + 'static) -> AppHandlerId {
        let id = self.resize_handlers.insert(Rc::new(f));

        AppHandlerId::Resize(id)
    }
}

impl Ctx for ApplicationBuilder {
    fn get_ctx(&self) -> &Context {
        &self.context
    }

    fn get_ctx_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

pub struct Application {
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    window_attributes: WindowAttributes,
    background: Color,
    mouse_position: IVec2,
    modifiers: Modifiers,
    focused: Option<ElementId>,
    elements: SlotMap<ElementId, Element>,
    pub(crate) ctx: Context,
    //----- Handlers -----//
    resize_handlers: SlotMap<ResizeId, Rc<dyn Fn(&mut Application, ResizeEvent)>>,
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
            background: Color::BLACK,
            resize_handlers: SlotMap::with_key(),
        })
    }

    pub fn create_signal<T: 'static>(&mut self, init: T) -> (ReadSignal<T>, WriteSignal<T>) {
        self.ctx.create_signal(init)
    }

    pub fn set_timeout(&mut self, delay: TimeDelta, f: impl FnOnce(&mut Application) + 'static) -> TimeoutId {
        self.ctx.set_timeout(delay, f)
    }

    pub fn clear_timeout(&mut self, id: TimeoutId) {
        self.ctx.clear_timeout(id);
    }

    pub fn set_interval(&mut self, delay: TimeDelta, f: impl Fn(&mut Application) + 'static) -> IntervalId {
        self.ctx.set_interval(delay, f)
    }

    pub fn clear_interval(&mut self, id: IntervalId) {
        self.ctx.clear_interval(id);
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

    pub fn get_background(&self) -> Color {
        self.background
    }

    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    pub fn on_resize(&mut self, f: impl Fn(&mut Application, ResizeEvent) + 'static) -> AppHandlerId {
        let id = self.resize_handlers.insert(Rc::new(f));

        AppHandlerId::Resize(id)
    }
}

impl Ctx for Application {
    fn get_ctx(&self) -> &Context {
        &self.ctx
    }

    fn get_ctx_mut(&mut self) -> &mut Context {
        &mut self.ctx
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
            WindowEvent::CursorMoved { device_id: _, position } => {
                let mouse_x = position.x as isize;
                let mouse_y = position.y as isize;

                let new_pos = IVec2::new(mouse_x, mouse_y);
                let delta = self.mouse_position - new_pos;

                self.mouse_position = new_pos;

                let ev = MouseMoveEvent {
                    pos: self.mouse_position,
                    modifiers: self.modifiers,
                    delta,
                };

                let keys: Vec<ElementId> = self.elements.keys().collect();
                for key in keys {
                    if let Some(el) = self.elements.get_mut(key) {
                        
                        if el.intersects(self.mouse_position) {
                            el.update(Event::MouseMove(ev));

                            let handlers: Vec<Rc<dyn Fn(&mut Application, ElementId, MouseMoveEvent)>> = el.handlers.mouse_move_handlers.values().cloned().collect();
                            for handler in handlers {
                                handler(self, key, ev);
                            }
                        }
                    }
                }

                self.surface.as_ref().expect("draw surface should exist").window().request_redraw();
            }
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
                    if let Some(el) = self.elements.get_mut(key) {
                        
                        if el.intersects(self.mouse_position) {
                            el.update(Event::Mouse(ev));

                            let handlers: Vec<Rc<dyn Fn(&mut Application, ElementId, MouseEvent)>> = el.handlers.mouse_handlers.values().cloned().collect();
                            for handler in handlers {
                                handler(self, key, ev);
                            }
                        }
                    }
                }

                self.surface.as_ref().expect("draw surface should exist").window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                println!("resized: {size:?}");

                let size_x = size.width as usize;
                let size_y = size.height as usize;

                let ev = ResizeEvent {
                    size: UVec2::new(size_x, size_y),
                };

                let handlers: Vec<Rc<dyn Fn(&mut Application, ResizeEvent)>> = self.resize_handlers.values().cloned().collect();
                for handler in handlers {
                    handler(self, ev);
                }

                let keys: Vec<ElementId> = self.elements.keys().collect();
                for key in keys {
                    if let Some(el) = self.elements.get_mut(key) {
                        el.update(Event::Resize(ev));
                        
                        let handlers: Vec<Rc<dyn Fn(&mut Application, ElementId, ResizeEvent)>> = el.handlers.resize_handlers.values().cloned().collect();
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

                if dimensions.width == 0 || dimensions.height == 0 {
                    return;
                }

                surface.resize(
                    NonZeroU32::new(dimensions.width).expect("window width should be greater than 0"),
                    NonZeroU32::new(dimensions.height).expect("window height should be greater than 0")
                ).expect("should be able to resize draw buffer");

                let mut window_buffer = surface.buffer_mut().expect("should be able to retrieve draw buffer");
                window_buffer.iter_mut().zip(buffer.into_iter()).for_each(|(current, write)| {
                    *current = write.into();
                });

                window_buffer.present().expect("should be able to present buffer");
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ProxyEvent) {
        match event {
            ProxyEvent::React => {
                self.ctx.clean();
                
                let keys: Vec<ElementId> = self.elements.keys().collect();
                for key in keys {
                    if let Some(el) = self.elements.get(key) {
                            let handlers: Vec<Rc<dyn Fn(&mut Application, ElementId)>> = el.handlers.rehydrate_handlers.values().cloned().collect();
                            for handler in handlers {
                                handler(self, key);
                        }
                    }
                }

                self.surface.as_ref().expect("window should exist").window().request_redraw();
            }
            ProxyEvent::Interval(id) => {
                if let Some(interval) = self.ctx.intervals.get(id) {
                    let callback = interval.f.clone();
                    callback(self);
                }

                self.surface.as_ref().expect("window should exist").window().request_redraw();
            }
            ProxyEvent::Timeout(id) => {
                // Remove in timeout instead of get in order to uphold
                // the requirement of `FnOnce`, as well as ensuring that
                // the timeout is only called once.
                if let Some(timeout) = self.ctx.timeouts.remove(id) {
                    let callback = timeout.f;
                    callback(self);
                }

                self.surface.as_ref().expect("window should exist").window().request_redraw();
            }
        }
    }
}
