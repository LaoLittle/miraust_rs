use std::ops::Deref;

use crate::managed::Managed;
use crate::message::chain::builder::MessageChainBuilder;
use crate::message::Message;
use crate::message::chain::iterator::MessageChainIter;

pub struct MessageChain {
    pub(crate) inner: Message,
}

impl MessageChain {
    pub fn builder() -> MessageChainBuilder {
        MessageChainBuilder::new()
    }

    pub fn iter(&self) -> MessageChainIter {
        self.into()
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

pub mod iterator {
    use crate::managed::Managed;
    use crate::message::chain::MessageChain;
    use crate::message::single::{SingleMessage};
    use crate::{RawPointer, RawPointerMut};

    pub struct MessageChainIter<'a> {
        _chain: &'a MessageChain,
        iter: Managed
    }

    impl<'a> From<&'a MessageChain> for MessageChainIter<'a> {
        fn from(chain: &'a MessageChain) -> Self {
            let r_iter = unsafe { message_chain_iterator(chain.0.pointer) };

            MessageChainIter {
                _chain: chain,
                iter: Managed::new(r_iter, 0)
            }
        }
    }

    impl<'a> Iterator for MessageChainIter<'a> {
        type Item = SingleMessage;

        fn next(&mut self) -> Option<Self::Item> {
            if unsafe { message_chain_iterator_has_next(self.iter.pointer) } {
                let m = Managed::new(unsafe { message_chain_iterator_next(self.iter.pointer) }, 0);

                let single = match unsafe { single_message_type(m.pointer) } {
                    1 => SingleMessage::PlainText(m.into()),
                    2 => SingleMessage::At,
                    3 => SingleMessage::AtAll,
                    4 => SingleMessage::Image(m.into()),
                    5 => SingleMessage::RichMessage,
                    6 => SingleMessage::Face,
                    7 => SingleMessage::ForwardMessage,
                    8 => SingleMessage::Audio,
                    9 => SingleMessage::MarketFace,
                    10 => SingleMessage::MusicShare,
                    _ => SingleMessage::Unknown
                };

                Some(single)
            } else { None }
        }
    }

    #[link(name = "miraust_core")]
    extern {
        fn message_chain_iterator(chain: RawPointer) -> RawPointerMut;

        fn message_chain_iterator_has_next(iter: RawPointer) -> bool;

        fn message_chain_iterator_next(iter: RawPointer) -> *mut ();

        fn single_message_type(single: RawPointer) -> i8;
    }
}

