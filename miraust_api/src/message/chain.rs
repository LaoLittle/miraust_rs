use std::ops::Deref;

use crate::managed::Managed;
use crate::message::chain::builder::MessageChainBuilder;
use crate::message::Message;

pub struct MessageChain {
    pub(crate) inner: Message,
}

impl MessageChain {
    pub fn builder() -> MessageChainBuilder {
        MessageChainBuilder::new()
    }

    pub fn iter(&self) -> MessageChainIter {
        MessageChainIter { chain: self }
    }
}

impl Deref for MessageChain {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ToString for MessageChain {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl From<Managed> for MessageChain {
    fn from(m: Managed) -> Self {
        Self { inner: m.into() }
    }
}

pub struct MessageChainIter<'a> {
    chain: &'a MessageChain,
}

impl<'a> Iterator for MessageChainIter<'a> {
    type Item = &'a MessageChain;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub mod builder {
    use crate::{RawPointer, RawPointerMut};
    use crate::managed::Managed;
    use crate::message::chain::MessageChain;
    use crate::message::single::{IntoSingleMessage, SingleMessage};

    pub struct MessageChainBuilder {
        chain: Vec<SingleMessage>,
    }

    impl MessageChainBuilder {
        pub fn new() -> MessageChainBuilder {
            Self::with_capacity(0)
        }

        pub fn with_capacity(capacity: usize) -> MessageChainBuilder {
            Self {
                chain: Vec::with_capacity(capacity),
            }
        }

        pub fn add<M: IntoSingleMessage>(&mut self, single: M) -> &mut MessageChainBuilder {
            self.chain.push(single.into_single_message());
            self
        }

        // pub fn add_text(&mut self, str: &str) {}

        pub fn build(&self) -> MessageChain {
            self.as_chain()
        }

        pub fn as_chain(&self) -> MessageChain {
            let self_len = self.chain.len();
            let len = if self_len > i32::MAX as usize { i32::MAX } else { self_len as i32 };

            let builder = Self::new_managed(len);

            for sm in self.chain.iter() {
                unsafe { message_chain_builder_add(builder.pointer, sm.inner_pointer()); }
            }

            Managed::new(unsafe { message_chain_builder_as_message_chain(builder.pointer) }, 0).into()
        }

        pub fn into_chain(self) -> MessageChain {
            let self_len = self.chain.len();
            let len = if self_len > i32::MAX as usize { i32::MAX } else { self_len as i32 };

            let builder = Self::new_managed(len);

            for sm in self.chain {
                unsafe { message_chain_builder_add(builder.pointer, sm.inner_pointer()); }
            }

            Managed::new(unsafe { message_chain_builder_as_message_chain(builder.pointer) }, 0).into()
        }

        fn new_managed(initial_size: i32) -> Managed {
            Managed::new(unsafe { new_message_chain_builder(initial_size) }, 0)
        }
    }

    impl Default for MessageChainBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    #[link(name = "miraust_core")]
    extern {
        fn new_message_chain_builder(initial_size: i32) -> RawPointerMut;

        fn message_chain_builder_add(chain: RawPointerMut, message: RawPointerMut) -> bool;

        fn message_chain_builder_as_message_chain(chain: RawPointer) -> RawPointerMut;
    }
}