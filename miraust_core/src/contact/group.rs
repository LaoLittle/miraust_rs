use jni::objects::GlobalRef;
use crate::contact::Contact;

pub struct Group {
    pub(crate) contact: Contact,
}