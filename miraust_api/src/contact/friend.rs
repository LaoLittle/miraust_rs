use std::ops::Deref;

use crate::contact::Contact;

pub struct Friend(pub(crate) Contact);

impl Deref for Friend {
    type Target = Contact;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}