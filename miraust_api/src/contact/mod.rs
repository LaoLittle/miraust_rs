use crate::{RawPointer, RawPointerMut};
use crate::managed::Managed;
use crate::message::Message;

pub mod friend;
pub mod group;
pub mod stranger;
pub mod member;

pub struct Contact(pub(crate) Managed);

impl Contact {
    pub fn id(&self) -> u64 {
        todo!()
    }

    pub fn send_message(&self, message: &Message) -> Self {
        let p = unsafe { contact_send_message(self.0.pointer, message.0.pointer) };

        Managed::new(p, 0).into()
    }
}

impl From<Managed> for Contact {
    fn from(m: Managed) -> Self {
        Self(m)
    }
}

#[link(name = "miraust_core")]
extern {
    fn contact_send_message(contact: RawPointer, message: RawPointer) -> RawPointerMut;
}