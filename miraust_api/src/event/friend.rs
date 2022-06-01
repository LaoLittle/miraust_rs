use crate::contact::friend::Friend;
use crate::event::{MessageEvent, MessageEventImpl};

pub struct FriendMessageEvent(pub(crate) MessageEventImpl);

impl FriendMessageEvent {
    fn subject(&self) -> Friend {
        Friend(self.0.subject())
    }
}