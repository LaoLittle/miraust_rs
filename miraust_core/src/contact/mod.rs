use jni::objects::GlobalRef;

pub mod friend;
pub mod group;
pub mod stranger;

pub struct Contact {
    pub(crate) inner: GlobalRef,
}