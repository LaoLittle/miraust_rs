use crate::event::{Event, MessageEventImpl};
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;

pub struct Listener {
    inner: Managed,
}

impl Listener {
    pub fn new<F: Fn(Event) + Send + 'static>(fun: F) -> Listener {
        let fun = Box::new(move |e: *mut (), t: u8| {
            let ma = Managed::new(e,0);
            let event_rs = match t {
                // 1 => Event::MessageEvent(MessageEvent { inner }),
                1 => Event::GroupMessageEvent(GroupMessageEvent(MessageEventImpl::from_managed(ma))),
                2 => Event::FriendMessageEvent(FriendMessageEvent(MessageEventImpl::from_managed(ma))),
                _ => Event::Any
            };

            fun(event_rs);
        });
        let ptr = unsafe { listener_subscribe_always(fun) };
        Listener { inner: Managed::new(ptr, 11) }
    }

    pub fn complete(&self) {
        unsafe { listener_stop(self.inner.pointer) };
    }
}

/*pub struct ListenerBuilder {
    listener: Listener,

}

impl ListenerBuilder {
    pub fn build(self) {

    }
}
*/
#[link(name = "miraust_core")]
extern {
    fn listener_subscribe_always(fun: Box<dyn Fn(*mut (), u8) + Send + 'static>) -> *mut ();

    fn listener_stop(listener: *const ());
}