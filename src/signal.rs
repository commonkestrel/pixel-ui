use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SignalId(pub(crate) usize);

pub struct Context {
    signals: RefCell<Vec<Node>>,
}

impl Context {
    fn create_signal<T: Any + 'static>(&self, init: T) -> (WriteSignal<T>, ReadSignal<T>) {
        let mut signals = self.signals.borrow_mut();
        let id = SignalId(signals.len());

        let content = Rc::new(RefCell::new(init)) as Rc<RefCell<dyn Any>>;
        let node = Node {
            signal: content,
            state: SignalState::Clean,
        };
        signals.push(node);

        (WriteSignal::new(id), ReadSignal::new(id))
    }

    fn try_with<T: 'static, U>(&self, id: SignalId, f: impl FnOnce(&T) -> U) -> U {
        f(self.signals.borrow()[id.0]
            .signal
            .borrow()
            .downcast_ref::<T>()
            .expect("should be able to downcast signal type"))
    }

    fn get_value(&self, id: SignalId) -> Option<Rc<RefCell<dyn Any>>> {
        self.signals
            .borrow()
            .get(id.0)
            .map(|node| node.signal.clone())
    }

    pub fn update<T: 'static>(&self, id: SignalId, f: impl FnOnce(&T) -> T) {
        let nodes = self.signals.borrow();
        let mut value = nodes[id.0].signal.borrow_mut();

        let update = f(value.downcast_ref().expect("should be able to downcast signal type"));
        *value.downcast_mut().expect("should be able to downcast signal type") = update;
    }
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
