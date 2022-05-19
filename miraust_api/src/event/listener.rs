use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::event::Event;

struct Header {}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Id(u64);

pub(crate) struct RawTask {
    ptr: NonNull<Header>,
}

pub struct Listener {
    raw: Option<RawTask>,
    id: Id,
    _p: PhantomData<()>,
}

impl Listener {
    pub fn new<F: Fn(Event) + Send + 'static>(fun: F) -> Listener {
        let fun = Box::new(fun);
        unsafe { event_spawn_subscribe_always(fun) }
    }

    pub fn complete(self) {
        unsafe { listener_stop(self) };
    }
}

#[link(name = "miraust_core")]
extern {
    fn event_spawn_subscribe_always(fun: Box<dyn Fn(Event) + Send + 'static>) -> Listener;

    fn listener_stop(handle: Listener);
}