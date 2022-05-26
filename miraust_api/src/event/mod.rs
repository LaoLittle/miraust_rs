use std::ops::Deref;
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

pub struct BaseEvent(pub(crate) Managed);

pub struct MessageEvent(pub(crate) BaseEvent);

impl MessageEvent {
    fn from_managed(m: Managed) -> MessageEvent {
        Self(BaseEvent(m))
    }

    pub fn subject(&self) -> Contact {
        let ptr = unsafe { message_event_get_subject(self.0.0.pointer) };

        Contact(Managed::new(ptr, 0))
    }

    pub fn message(&self) -> MessageChain {
        let ptr = unsafe { message_event_get_message(self.0.0.pointer) };

        MessageChain { m: Message(Managed::new(ptr, 0)) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_event_get_subject(event: *const ()) -> *mut ();

    fn message_event_get_message(event: *const ()) -> *mut ();
}