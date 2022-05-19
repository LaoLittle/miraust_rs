use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::event::Event;

struct Header {}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(u64);

pub(crate) struct RawTask {
    _ptr: NonNull<Header>,
}

pub struct Listener {
    _raw: Option<RawTask>,
    _id: Id,
    _p: PhantomData<()>,
}

impl Listener {
    pub fn new<F: Fn(Event) + Send + 'static>(fun: F) -> Listener {
        let fun = Box::new(fun);
        unsafe { listener_subscribe_always(fun) }
    }

    pub fn complete(self) {
        unsafe { listener_stop(self) };
    }
}

#[link(name = "miraust_core")]
extern {
    fn listener_subscribe_always(fun: Box<dyn Fn(Event) + Send + 'static>) -> Listener;

    fn listener_stop(listener: Listener);
}