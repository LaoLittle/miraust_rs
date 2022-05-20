use jni::objects::GlobalRef;

pub struct Friend {
    pub(crate) inner: GlobalRef,
}

impl Friend {
    pub const fn id(&self) -> u64 {
        todo!()
    }
}