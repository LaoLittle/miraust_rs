use jni::objects::GlobalRef;

pub struct Stranger {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}