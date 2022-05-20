use crate::contact::Contact;

use crate::managed::Managed;

pub struct Stranger {
    pub(crate) inner: Managed,
}

impl Contact for Stranger {
    fn id(&self) -> u64 {
        todo!()
    }

    fn send_message(&self) {
        todo!()
    }
}