use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::event::Event;
use crate::managed::Managed;

pub struct Listener {
    inner: Managed,
}

impl Listener {
    pub fn new<F: Fn(Event) + Send + 'static>(fun: F) -> Listener {
        let fun = Box::new(fun);
        let ptr = unsafe { listener_subscribe_always(fun) };
        Listener { inner: Managed::new(ptr, 11) }
    }

    pub fn complete(&self) {
        unsafe { listener_stop(self.inner.pointer) };
    }
}

#[link(name = "miraust_core")]
extern {
    fn listener_subscribe_always(fun: Box<dyn Fn(Event) + Send + 'static>) -> *mut ();

    fn listener_stop(listener: *const ());
}