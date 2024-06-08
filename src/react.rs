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
    signals: SlotMap<SignalId, Node>,
    timeouts: SlotMap<TimeoutId, Timeout>,
    intervals: SlotMap<IntervalId, Interval>,
}

impl Context {
    pub(crate) fn new(ev: EventLoopProxy<ProxyEvent>) -> Self {
        Context {
            event_loop: ev,
            timer: Timer::new(),
            signals: SlotMap::with_key(),
            timeouts: SlotMap::with_key(),
            intervals: SlotMap::with_key(),
        }
    }

    pub fn create_signal<T: Any + 'static>(&mut self, init: T) -> (WriteSignal<T>, ReadSignal<T>) {
        let content = Rc::new(RefCell::new(init)) as Rc<RefCell<dyn Any>>;
        let node = Node {
            signal: content,
            state: SignalState::Clean,
        };
        let id = self.signals.insert(node);

        (WriteSignal::new(id), ReadSignal::new(id))
    }

    pub fn set_timeout(&mut self, delay: TimeDelta, f: impl FnOnce(&mut Application) + 'static) -> TimeoutId {
        self.timeouts.insert_with_key(|id| {
            let proxy = self.event_loop.clone();

            let handle = self.timer.schedule_with_delay(delay, move || {
                proxy.send_event(ProxyEvent::Timeout(id)).expect("event loop should still be active");
            });

            Timeout {
                handle,
                f: Box::new(f),
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
                handle,
                f: Box::new(f),
            }
        })
    }

    pub fn clear_interval(&mut self, id: IntervalId) {
        let _ = self.intervals.remove(id);
    }

    fn try_with<T: 'static, U>(&self, id: SignalId, f: impl FnOnce(&T) -> U) -> U {
        f(self.signals
            .get(id)
            .expect("signal id should still exist")
            .signal
            .borrow()
            .downcast_ref::<T>()
            .expect("should be able to downcast signal type"))
    }

    fn get_value(&self, id: SignalId) -> Option<Rc<RefCell<dyn Any>>> {
        self.signals
            .get(id)
            .map(|node| node.signal.clone())
    }

    pub fn update<T: 'static>(&self, id: SignalId, f: impl FnOnce(&T) -> T) {
        let mut value = self.signals[id].signal.borrow_mut();

        let update = f(value.downcast_ref().expect("should be able to downcast signal type"));
        *value.downcast_mut().expect("should be able to downcast signal type") = update;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProxyEvent {
    Timeout(TimeoutId),
    Interval(IntervalId),
}

struct Node {
    signal: Rc<RefCell<dyn Any>>,
    state: SignalState,
}

pub(crate) enum SignalState {
    Clean,
    Dirty,
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
    pub fn set(&self, ctx: &Context, data: T) {
        ctx.update(self.id, |_| data);
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
    pub fn get(&self, ctx: &Context) -> T {
        ctx.try_with(self.id, T::clone)
    }
}

struct Timeout {
    f: Box<dyn FnOnce(&mut Application) + 'static>,
    handle: Guard,
}

struct Interval {
    f: Box<dyn Fn(&mut Application) + 'static>,
    handle: Guard,
}
