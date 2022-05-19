use jni::objects::GlobalRef;

pub struct Friend {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl Friend {
    pub const fn id(&self) -> u64 {
        self.id as u64
    }


    pub fn a(&self) {
        let _ = self.inner;
    }
}