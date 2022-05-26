use crate::managed::Managed;

pub struct Message(pub(crate) Managed);

impl Message {}

impl ToString for Message {
    fn to_string(&self) -> String {
        unsafe { message_to_string(self.0.pointer) }
    }
}

pub struct MessageChain {
    pub(crate) m: Message,
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