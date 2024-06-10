use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};
use chrono::{Duration, TimeDelta};
use slotmap::{new_key_type, SlotMap};
use timer::{Guard, Timer};
use winit::event_loop::EventLoopProxy;

use crate::app::Application;

new_key_type! {
    pub struct SignalId;
    
    pub struct TimeoutId;

    pub struct IntervalId;
}

pub(crate) struct Context {
    event_loop: EventLoopProxy<ProxyEvent>,
    timer: Timer,
    signals: SlotMap<SignalId, Box<dyn Any>>,
    pub(crate) timeouts: SlotMap<TimeoutId, Timeout>,
    pub(crate) intervals: SlotMap<IntervalId, Interval>,
    clean: bool,
}

/// A trait used so that both [`ApplicationBuilder`](`crate::app::ApplicationBuilder`)
/// and [`Application`](`crate::app::Application`) can be used with signals.
pub(crate) trait Ctx {
    // Keep Context private so that Ctx cannot be implemented on any external types.
    fn get_ctx(&self) -> &Context;
    fn get_ctx_mut(&mut self) -> &mut Context;
}

impl Context {
    pub(crate) fn new(ev: EventLoopProxy<ProxyEvent>) -> Self {
        Context {
            event_loop: ev,
            timer: Timer::new(),
            signals: SlotMap::with_key(),
            timeouts: SlotMap::with_key(),
            intervals: SlotMap::with_key(),
            clean: true,
        }
    }

    pub(crate) fn clean(&mut self) {
        self.clean = true;
    }

    pub fn create_signal<T: Any + 'static>(&mut self, init: T) -> (ReadSignal<T>, WriteSignal<T>) {
        let content = Box::new(init) as Box<dyn Any>;
        let id = self.signals.insert(content);

        (ReadSignal::new(id), WriteSignal::new(id))
    }

    pub fn set_timeout(&mut self, delay: TimeDelta, f: impl FnOnce(&mut Application) + 'static) -> TimeoutId {
        self.timeouts.insert_with_key(|id| {
            let proxy = self.event_loop.clone();

            let handle = self.timer.schedule_with_delay(delay, move || {
                proxy.send_event(ProxyEvent::Timeout(id)).expect("event loop should still be active");
            });

            Timeout {
                f: Box::new(f),
                __guard: handle,
            }
        })
    }

    pub fn clear_timeout(&mut self, id: TimeoutId) {
        let _ = self.timeouts.remove(id);
    }

    pub fn set_interval(&mut self, delay: TimeDelta, f: impl Fn(&mut Application) + 'static) -> IntervalId {
        self.intervals.insert_with_key(|id| {
            let proxy = self.event_loop.clone();

            let handle = self.timer.schedule_repeating(delay, move || {
                proxy.send_event(ProxyEvent::Interval(id)).expect("event loop should still be active");
            });

            Interval {
                f: Rc::new(f),
                __guard: handle,
            }
        })
    }

    pub fn clear_interval(&mut self, id: IntervalId) {
        let _ = self.intervals.remove(id);
    }

    fn try_with<T: 'static, R>(&self, id: SignalId, f: impl FnOnce(&T) -> R) -> R {
        f(self.signals
            .get(id)
            .expect("signal id should still exist")
            .downcast_ref::<T>()
            .expect("should be able to downcast signal type"))
    }
    
    pub fn update<T: 'static>(&mut self, id: SignalId, f: impl FnOnce(&mut T)) {
        f(self.signals[id].downcast_mut().expect("should be able to downcast signal type"));

        if self.clean {
            self.clean = false;
            self.event_loop.send_event(ProxyEvent::React).expect("event loop should still be active");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProxyEvent {
    Timeout(TimeoutId),
    Interval(IntervalId),
    React,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteSignal<T> {
    id: SignalId,
    _marker: PhantomData<T>,
}

impl<T> WriteSignal<T> {
    fn new(id: SignalId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl <T: 'static> WriteSignal<T> {
    pub fn set<A: Ctx>(&self, app: &mut A, data: T) {
        app.get_ctx_mut().update(self.id, |sig| *sig = data);
    }

    pub fn update<A: Ctx>(&self, app: &mut A, f: impl FnOnce(&mut T)) {
        app.get_ctx_mut().update(self.id, f);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadSignal<T> {
    id: SignalId,
    _marker: PhantomData<T>,
}

impl<T> ReadSignal<T> {
    fn new(id: SignalId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + 'static> ReadSignal<T> {
    pub fn get<A: Ctx>(&self, app: &A) -> T {
        app.get_ctx().try_with(self.id, T::clone)
    }

    pub fn with<A: Ctx, R>(&self, app: &A, f: impl FnOnce(&T) -> R) -> R {
        app.get_ctx().try_with(self.id, f)
    }
}

pub(crate) struct Timeout {
    pub(crate) f: Box<dyn FnOnce(&mut Application) + 'static>,
    __guard: Guard,
}

pub(crate) struct Interval {
    pub(crate) f: Rc<dyn Fn(&mut Application) + 'static>,
    __guard: Guard,
}
