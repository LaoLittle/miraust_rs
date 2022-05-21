use crate::contact::group::Group;
use crate::event::MessageEvent;
use crate::message::{Message, MessageChain};


pub struct GroupMessageEvent {
    pub(crate) e: MessageEvent,
}

impl GroupMessageEvent {
    pub fn subject(&self) -> Group {
        Group { contact: self.e.subject() }
    }

    pub fn message(&self) -> MessageChain {
        self.e.message()
    }
}