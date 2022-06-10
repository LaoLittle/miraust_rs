use std::ops::Deref;

use crate::contact::Contact;

pub struct Stranger(pub(crate) Contact);

impl Deref for Stranger {
    type Target = Contact;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}