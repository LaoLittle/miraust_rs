use crate::event::{Event, EventManaged, MessageEvent};
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;

pub struct Listener {
    inner: Managed,
}

impl Listener {
    pub fn new<F: Fn(Event) + Send + 'static>(fun: F) -> Listener {
        let fun = Box::new(move |e: EventManaged| {
            let inner = e.inner;
            let event_rs = match e.event_type {
                // 1 => Event::MessageEvent(MessageEvent { inner }),
                1 => Event::GroupMessageEvent(GroupMessageEvent { e: MessageEvent { inner } }),
                2 => Event::FriendMessageEvent(FriendMessageEvent { e: MessageEvent { inner } }),
                _ => Event::Any(EventManaged { inner, event_type: e.event_type })
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
    fn listener_subscribe_always(fun: Box<dyn Fn(EventManaged) + Send + 'static>) -> *mut ();

    fn listener_stop(listener: *const ());
}