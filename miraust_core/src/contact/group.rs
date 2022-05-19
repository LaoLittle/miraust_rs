use jni::objects::GlobalRef;

pub struct Group {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}