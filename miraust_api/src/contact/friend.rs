use crate::jni_struct::GlobalRef;

pub struct Friend {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}