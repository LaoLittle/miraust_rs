use crate::contact::friend::Friend;
use crate::event::MessageEvent;

pub struct FriendMessageEvent {
    pub(crate) e: MessageEvent,
}

impl FriendMessageEvent {
    fn subject(&self) -> Friend {
        Friend { contact: self.e.subject() }
    }
}