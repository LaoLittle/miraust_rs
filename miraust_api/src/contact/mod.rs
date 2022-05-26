use crate::managed::Managed;

pub mod friend;
pub mod group;
pub mod stranger;
pub mod member;

pub struct Contact(pub(crate) Managed);