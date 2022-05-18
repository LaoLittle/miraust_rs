use crate::jni_struct::GlobalRef;

pub struct Stranger {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}