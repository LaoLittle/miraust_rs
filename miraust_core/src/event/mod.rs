use jni::objects::GlobalRef;

#[derive(Clone)]
pub struct Event {
    pub(crate) inner: GlobalRef
}