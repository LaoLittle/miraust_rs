use crate::contact::Contact;
use crate::event::Event;

pub(crate) struct MessageEvent {
    event: Event
}

impl MessageEvent  {
    fn subject<C: Contact>(&self) -> C {
        todo!()
    }

    // fn sender(&self) -> Self::Sender;
}