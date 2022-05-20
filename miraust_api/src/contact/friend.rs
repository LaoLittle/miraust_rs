use crate::contact::Contact;
use crate::managed::Managed;

pub struct Friend {
    pub(crate) inner: Managed
}

impl Contact for Friend {
    fn id(&self) -> u64 {
        todo!()
    }

    fn send_message(&self) {
        todo!()
    }
}