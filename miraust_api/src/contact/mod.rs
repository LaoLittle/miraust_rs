pub mod friend;
pub mod group;
pub mod stranger;

pub trait Contact {
    fn id(&self) -> u64;

    fn send_message(&self);
}