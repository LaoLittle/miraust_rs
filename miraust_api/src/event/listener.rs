use crate::{RawPointer, RawPointerMut};
use crate::event::Event;
use crate::managed::Managed;

pub struct Listener {
    inner: Managed,
}

#[repr(C)]
struct ListenerInvoke {
    fun: *mut (),
    invoke: fn(*const ()),
    drop: fn(*mut ()),
}

impl Listener {
    pub fn new<F>(fun: F) -> Listener
        where F: Fn(Event) + Send + 'static
    {
        let fun = Box::new(move |e: RawPointerMut, t: u8| {
            let ma = Managed::new(e, 0);
            let event_rs = match t {
                // 1 => Event::MessageEvent(MessageEvent { inner }),
                1 => Event::GroupMessageEvent(ma.into()),
                2 => Event::FriendMessageEvent(ma.into()),
                _ => Event::Any
            };

            fun(event_rs);
        });

        let ptr = unsafe { listener_subscribe_always(fun) };
        Listener { inner: Managed::new(ptr, 11) }
    }

    pub fn complete(&self) {
        unsafe { listener_abort(self.inner.pointer) };
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        self.complete();
    }
}

#[link(name = "miraust_core")]
extern {
    fn listener_subscribe_always(fun: Box<dyn Fn(RawPointerMut, u8) + Send + 'static>) -> RawPointerMut;

    fn listener_abort(listener: RawPointer);
}