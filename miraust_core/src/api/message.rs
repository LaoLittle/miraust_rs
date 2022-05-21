use std::sync::mpsc;

use jni::objects::GlobalRef;

use crate::jni_ffi::jni_callback::call_back;
use crate::message::Message;

#[no_mangle]
extern fn message_to_string(message: &GlobalRef) -> String {
    message_to_string0(message).expect("Cannot get message")
}

fn message_to_string0(message: &GlobalRef) -> Option<String> {
    let global_ref = message.clone();

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Message::to_string_unchecked(global_ref, env) }).unwrap();
    });

    recv.recv().unwrap()
}