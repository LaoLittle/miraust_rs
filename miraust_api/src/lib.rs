pub mod bot;
mod jni_struct;
pub mod contact;
pub mod plugin;
pub mod event;

#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use crate::jni_struct::{GlobalRef};

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
