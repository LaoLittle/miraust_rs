use std::ops::Deref;

use crate::managed::Managed;

mod builder;
mod single;

pub struct Message(pub(crate) Managed);

impl Message {
    pub fn content(&self) -> String {
        unsafe { message_content_to_string(self.0.pointer) }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        unsafe { message_to_string(self.0.pointer) }
    }
}

impl From<MessageChain> for Message {
    fn from(chain: MessageChain) -> Self {
        chain.inner
    }
}

pub struct MessageChain {
    pub(crate) inner: Message,
}

impl Deref for MessageChain {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ToString for MessageChain {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

#[link(name = "miraust_core")]
extern {
    fn message_to_string(message: *const ()) -> String;

    fn message_content_to_string(message: *const ()) -> String;
}