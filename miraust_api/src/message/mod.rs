use crate::{RawPointer, RawString};
use crate::managed::Managed;
use crate::message::chain::MessageChain;

pub mod chain;
pub mod single;

pub struct Message(pub(crate) Managed);

impl Message {
    pub fn content(&self) -> String {
        unsafe { message_content_to_string(self.0.pointer).into() }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        unsafe { message_to_string(self.0.pointer).into() }
    }
}

impl From<MessageChain> for Message {
    fn from(chain: MessageChain) -> Self {
        chain.inner
    }
}

impl From<Managed> for Message {
    fn from(m: Managed) -> Self {
        Self(m)
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_to_string(message: RawPointer) -> RawString;

    fn message_content_to_string(message: RawPointer) -> RawString;
}