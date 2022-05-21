use crate::managed::Managed;

pub struct Message {
    pub(crate) inner: Managed
}

impl Message {

}

impl ToString for Message {
    fn to_string(&self) -> String {
        unsafe { message_to_string(self.inner.pointer) }
    }
}

pub struct MessageChain {
    pub(crate) m: Message
}

impl ToString for MessageChain {
    fn to_string(&self) -> String {
        self.m.to_string()
    }
}


#[link(name = "miraust_core")]
extern {
    fn message_to_string(message: *const ()) -> String;
}