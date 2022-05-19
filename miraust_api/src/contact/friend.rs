use crate::contact::Contact;
use crate::jni_struct::GlobalRef;

pub struct Friend {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl Contact for Friend {
    fn id(&self) -> u64 {
        self.id as _
    }

    fn send_message(&self) {
        todo!()
    }
}