use crate::contact::group::Group;
use crate::event::MessageEvent;
use crate::message::{Message, MessageChain};

pub struct GroupMessageEvent(pub(crate) MessageEvent);

impl GroupMessageEvent {
    pub fn subject(&self) -> Group {
        Group(self.0.subject())
    }

    pub fn message(&self) -> MessageChain {
        self.0.message()
    }
}