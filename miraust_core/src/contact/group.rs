use jni::objects::GlobalRef;
use crate::contact::Contact;

pub struct Group {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl Contact for Group {
    fn id(&self) -> u64 {
        self.id as u64
    }
}