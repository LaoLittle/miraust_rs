use crate::contact::group::Group;
use crate::event::{MessageEvent, MessageEventImpl};
use crate::message::{MessageChain};

pub struct GroupMessageEvent(pub(crate) MessageEventImpl);

impl GroupMessageEvent {
    pub fn subject(&self) -> Group {
        Group(self.0.subject())
    }

    pub fn message(&self) -> MessageChain {
        self.0.message()
    }
}