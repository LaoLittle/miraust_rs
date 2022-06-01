use crate::message::Message;
use crate::message::single::SingleMessage;

pub struct MessageChainBuilder {
    chain: Vec<Message>
}

impl MessageChainBuilder {
    pub fn new() -> MessageChainBuilder {
        Self {
            chain: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> MessageChainBuilder {
        Self {
            chain: Vec::with_capacity(capacity)
        }
    }

    pub fn add(&self, single: SingleMessage) {

    }

    pub fn add_text(&self, text: &str) {

    }

    pub fn build(&self) {

    }
}

impl Default for MessageChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}