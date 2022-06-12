use crate::{RawPointer, RawPointerMut};
use crate::contact::Contact;
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;
use crate::message::Message;
use crate::message::chain::MessageChain;

pub mod listener;
pub mod group;
mod friend;

pub enum Event {
    GroupMessageEvent(GroupMessageEvent),
    FriendMessageEvent(FriendMessageEvent),
    Any,
}

pub struct BaseEvent(pub(crate) Managed);

pub struct MessageEvent(pub(crate) BaseEvent);

impl MessageEvent {
    pub fn subject(&self) -> Contact {
        let ptr = unsafe { message_event_get_subject(self.0.0.pointer) };

        Contact(Managed::new(ptr, 0))
    }

    pub fn message(&self) -> MessageChain {
        let ptr = unsafe { message_event_get_message(self.0.0.pointer) };

        MessageChain { inner: Message(Managed::new(ptr, 0)) }
    }
}

impl From<Managed> for MessageEvent {
    fn from(m: Managed) -> Self {
        Self(BaseEvent(m))
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_event_get_subject(event: RawPointer) -> RawPointerMut;

    fn message_event_get_message(event: RawPointer) -> RawPointerMut;
}