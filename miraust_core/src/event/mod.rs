use jni::objects::GlobalRef;
use crate::managed::Managed;

#[derive(Clone)]
pub struct Event {
    pub(crate) inner: GlobalRef
}

#[repr(C)]
#[derive(Debug)]
pub struct EventManaged {
    pub(crate) inner: Managed
}