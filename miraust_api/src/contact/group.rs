use crate::jni_struct::GlobalRef;

pub struct Group {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}