use std::ops::Deref;

use crate::contact::Contact;

pub struct Group(pub(crate) Contact);

impl Deref for Group {
    type Target = Contact;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}