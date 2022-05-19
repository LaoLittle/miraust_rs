pub mod friend;
pub mod group;
pub mod stranger;
pub mod member;

pub trait Contact {
    fn id(&self) -> u64;

    fn send_message(&self);
}