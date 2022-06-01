
use crate::contact::{Contact};
use crate::event::friend::FriendMessageEvent;
use crate::event::group::GroupMessageEvent;
use crate::managed::Managed;
use crate::message::{Message, MessageChain};

pub mod listener;
pub mod group;
mod friend;

pub enum Event {
    GroupMessageEvent(GroupMessageEvent),
    FriendMessageEvent(FriendMessageEvent),
    Any,
}

pub struct BaseEvent(pub(crate) Managed);

pub trait MessageEvent {
    fn subject(&self) -> Contact;

    fn message(&self) -> MessageChain;
}

pub(crate) struct MessageEventImpl(pub(crate) BaseEvent);

impl MessageEventImpl {
    fn from_managed(m: Managed) -> MessageEventImpl {
        Self(BaseEvent(m))
    }
}

impl MessageEvent for MessageEventImpl {
    fn subject(&self) -> Contact {
        println!("MessageEvent::subject");
        let ptr = unsafe { message_event_get_subject(self.0.0.pointer) };

        Contact(Managed::new(ptr, 0))
    }

    fn message(&self) -> MessageChain {
        let ptr = unsafe { message_event_get_message(self.0.0.pointer) };

        MessageChain { m: Message(Managed::new(ptr, 0)) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_event_get_subject(event: *const ()) -> *mut ();

    fn message_event_get_message(event: *const ()) -> *mut ();
}