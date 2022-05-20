use crate::contact::Contact;
use crate::managed::Managed;

pub struct Group {
    pub(crate) inner: Managed,
}

impl Contact for Group {
    fn id(&self) -> u64 {
        todo!()
    }

    fn send_message(&self) {
        todo!()
    }
}