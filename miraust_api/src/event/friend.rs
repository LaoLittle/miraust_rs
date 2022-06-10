use std::ops::Deref;

use crate::contact::friend::Friend;
use crate::event::MessageEvent;
use crate::managed::Managed;

pub struct FriendMessageEvent(pub(crate) MessageEvent);

impl FriendMessageEvent {
    pub fn subject(&self) -> Friend {
        Friend(self.0.subject())
    }
}

impl Deref for FriendMessageEvent {
    type Target = MessageEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Managed> for FriendMessageEvent {
    fn from(m: Managed) -> Self {
        Self(m.into())
    }
}