use jni::objects::GlobalRef;
use crate::contact::Contact;

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

impl Contact for Friend {
    fn id(&self) -> u64 {
        self.id as u64
    }
}