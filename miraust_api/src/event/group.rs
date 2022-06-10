use std::ops::Deref;

use crate::contact::group::Group;
use crate::event::MessageEvent;
use crate::managed::Managed;

pub struct GroupMessageEvent(pub(crate) MessageEvent);

impl GroupMessageEvent {
    pub fn subject(&self) -> Group {
        Group(self.0.subject())
    }
}

impl Deref for GroupMessageEvent {
    type Target = MessageEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Managed> for GroupMessageEvent {
    fn from(m: Managed) -> Self {
        Self(m.into())
    }
}