use crate::jni_struct::GlobalRef;

pub mod group_event;
pub mod message_event;
pub mod friend_event;
pub mod listener;

pub struct Event {
    inner: GlobalRef
}

trait HasSubject {

}