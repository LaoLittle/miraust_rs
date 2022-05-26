use crate::contact::friend::Friend;
use crate::event::{MessageEvent};

pub struct FriendMessageEvent(pub(crate) MessageEvent);

impl FriendMessageEvent {
    fn subject(&self) -> Friend {
        Friend(self.0.subject())
    }
}