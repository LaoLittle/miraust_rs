use crate::contact::Contact;
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;

pub mod listener;
pub mod group;
mod friend;

#[derive(Debug)]
#[repr(C)]
pub struct EventManaged {
    pub(crate) inner: Managed,
    pub(crate) event_type: u8,
}

pub enum Event {
    MessageEvent(MessageEvent),
    GroupMessageEvent(GroupMessageEvent),
    FriendMessageEvent(FriendMessageEvent),
    Any(EventManaged),
}

pub struct MessageEvent {
    pub(crate) inner: Managed,
}

impl MessageEvent {
    fn subject(&self) -> Contact {
        let ptr = unsafe { message_event_get_subject(self.inner.pointer) };

        Contact { inner: Managed::new(ptr, 20) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_event_get_subject(event: *const ()) -> *mut ();
}