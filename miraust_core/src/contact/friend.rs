use jni::objects::GlobalRef;
use crate::contact::Contact;

pub struct Friend {
    pub(crate) contact: Contact,
}

impl Friend {
    pub const fn id(&self) -> u64 {
        todo!()
    }
}