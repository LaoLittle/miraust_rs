#![feature(vec_into_raw_parts)]

pub mod bot;
pub mod contact;
pub mod plugin;
pub mod event;
pub(crate) mod managed;
pub mod message;

pub type RawPointer = *const ();
pub type RawPointerMut = *mut ();

#[repr(C)]
pub struct RawString {
    pointer: *mut u8,
    length: usize,
    capacity: usize,
}

impl From<String> for RawString {
    fn from(s: String) -> Self {
        let (ptr, len, cap) = s.into_raw_parts();
        Self {
            pointer: ptr,
            length: len,
            capacity: cap,
        }
    }
}

impl From<RawString> for String {
    fn from(raw: RawString) -> Self {
        unsafe { String::from_raw_parts(raw.pointer, raw.length, raw.capacity) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn length() {
        println!("{}", 1);
    }
}
