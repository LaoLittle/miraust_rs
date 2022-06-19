#![feature(once_cell)]
#![feature(vec_into_raw_parts)]

use std::slice;

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

impl RawString {
    pub fn is_null(&self) -> bool {
        self.pointer.is_null()
    }

    pub fn get_str(&self) -> Option<&str> {
        if self.is_null() { return None; }

        let str = unsafe {
            let bytes: &[u8] = slice::from_raw_parts(self.pointer, self.length);
            std::str::from_utf8_unchecked(bytes)
        };

        Some(str)
    }

    pub fn string(self) -> Option<String> {
        if self.is_null() { return None; }
        let s = unsafe { String::from_raw_parts(self.pointer, self.length, self.capacity) };
        Some(s)
    }
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

impl AsRef<str> for RawString {
    fn as_ref(&self) -> &str {
        self.get_str().unwrap()
    }
}