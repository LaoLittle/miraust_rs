use crate::contact::Contact;
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;
use crate::message::{Message, MessageChain};

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

        Contact { inner: Managed::new(ptr, 0) }
    }

    fn message(&self) -> MessageChain {
        let ptr = unsafe { message_event_get_subject(self.inner.pointer) };

        MessageChain { m: Message { inner: Managed::new(ptr, 0) } }
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_event_get_subject(event: *const ()) -> *mut ();

    fn message_event_get_message(event: *const ()) -> *mut ();
}