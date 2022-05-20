use crate::managed::Managed;

pub mod group_event;
pub mod message_event;
pub mod friend_event;
pub mod listener;

#[derive(Debug)]
#[repr(C)]
pub struct Event {
    pub(crate) inner: Managed
}

trait HasSubject {

}