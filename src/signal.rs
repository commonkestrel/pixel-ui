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
        f(self.signals.borrow()[id.0].signal.borrow().downcast_ref::<T>().expect("should be able to downcast signal type"))
    }

    fn get_value(&self, subscription: SignalId) -> Option<Rc<RefCell<dyn Any>>> {
        self.signals.borrow().get(subscription.0).map(|node| node.signal.clone())
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

pub struct WriteSignal<T> {
    id: SignalId,
    _marker: PhantomData<T>
}

impl<T> WriteSignal<T> {
    fn new(id: SignalId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn set(&self, ctx: &Context, data: T) {
        
    }
}

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
