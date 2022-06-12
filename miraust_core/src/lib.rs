#![feature(once_cell)]
#![feature(vec_into_raw_parts)]

use tokio::task::JoinHandle;

use crate::plugin::RustPlugin;

mod test;
mod plugin;
mod plugin_loader;
mod plugin_manager;
mod jni_ffi;
mod bot;
mod contact;
mod api;
mod event;
mod managed;
mod message;

pub type Listener = JoinHandle<()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawString {
    pointer: *mut u8,
    length: usize,
    capacity: usize,
}

unsafe impl Send for RawString {}

impl From<(*mut u8, usize, usize)> for RawString {
    fn from(raw_parts: (*mut u8, usize, usize)) -> Self {
        Self {
            pointer: raw_parts.0,
            length: raw_parts.1,
            capacity: raw_parts.2,
        }
    }
}

impl From<String> for RawString {
    fn from(s: String) -> Self {
        s.into_raw_parts().into()
    }
}

impl From<RawString> for String {
    fn from(raw: RawString) -> Self {
        unsafe { String::from_raw_parts(raw.pointer, raw.length, raw.capacity) }
    }
}