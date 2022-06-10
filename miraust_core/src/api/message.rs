use jni::objects::GlobalRef;

use crate::jni_ffi::jni_callback::jni_call_back;

#[no_mangle]
extern fn message_to_string(message: &GlobalRef) -> String {
    message_to_string0(message).expect("Error on Message.toString()")
}

fn message_to_string0(message: &GlobalRef) -> Option<String> {
    let global_ref = message.clone();

    jni_call_back(|env| {
        unsafe { crate::message::to_string_unchecked(env, global_ref) }
    })
}

#[no_mangle]
extern fn message_content_to_string(message: &GlobalRef) -> String {
    message_content_to_string0(message).expect("Error on Message.contentToString()")
}

fn message_content_to_string0(message: &GlobalRef) -> Option<String> {
    let global_ref = message.clone();

    jni_call_back(|env| {
        unsafe { crate::message::content_to_string_unchecked(env, global_ref) }
    })
}