use jni::objects::GlobalRef;

pub struct Contact {
    pub(crate) inner: GlobalRef,
}