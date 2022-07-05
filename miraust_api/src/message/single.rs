use std::ops::Deref;

use crate::contact::member::Member;
use crate::message::Message;
use crate::RawPointerMut;

pub enum SingleMessage {
    PlainText(imp::PlainText),
    At,
    AtAll,
    Image(imp::Image),
    RichMessage,
    //ServiceMessage,
    Face,
    ForwardMessage,
    Audio,
    MarketFace,
    MusicShare,
    Unknown
}

impl SingleMessage {
    pub fn at(id: u64) -> Self {
        SingleMessage::At
    }

    pub(crate) fn inner_pointer(&self) -> RawPointerMut {
        match self {
            SingleMessage::PlainText(p) => p.0.pointer,
            SingleMessage::Image(i) => i.0.pointer,
            _ => todo!()
        }
    }
}

impl Deref for SingleMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        match self {
            SingleMessage::PlainText(p) => p,
            SingleMessage::Image(i) => i,
            _ => todo!()
        }
    }
}

impl From<String> for SingleMessage {
    fn from(s: String) -> Self {
        Self::PlainText(s.into())
    }
}

impl From<Member> for SingleMessage {
    fn from(m: Member) -> Self {
        Self::At
    }
}

impl From<u64> for SingleMessage {
    fn from(id: u64) -> Self {
        Self::At
    }
}

pub trait IntoSingleMessage {
    fn into_single_message(self) -> SingleMessage;
}

impl IntoSingleMessage for SingleMessage {
    fn into_single_message(self) -> SingleMessage {
        self
    }
}

impl IntoSingleMessage for String {
    fn into_single_message(self) -> SingleMessage {
        SingleMessage::PlainText(self.into())
    }
}

impl IntoSingleMessage for &str {
    fn into_single_message(self) -> SingleMessage {
        self.to_string().into_single_message()
    }
}

pub mod imp {
    use std::ops::Deref;

    use crate::{RawPointerMut, RawString};
    use crate::managed::Managed;
    use crate::message::Message;

    pub struct PlainText {
        pub(crate) inner: Message,
    }

    impl Deref for PlainText {
        type Target = Message;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl From<Managed> for PlainText {
        fn from(m: Managed) -> Self {
            Self { inner: m.into() }
        }
    }

    impl From<String> for PlainText {
        fn from(s: String) -> Self {
            let ptr = unsafe { new_plain_text(s.into()) };

            Managed::new(ptr, 0).into()
        }
    }

    impl From<&str> for PlainText {
        fn from(s: &str) -> Self {
            Self::from(s.to_string())
        }
    }

    pub struct Image {
        pub(crate) inner: Message,
    }

    impl From<Managed> for Image {
        fn from(m: Managed) -> Self {
            Self { inner: m.into() }
        }
    }

    impl Deref for Image {
        type Target = Message;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    #[link(name = "miraust_core")]
    extern {
        fn new_plain_text(str: RawString) -> RawPointerMut;
    }
}