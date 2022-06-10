use jni::objects::GlobalRef;

use crate::jni_ffi::jni_callback::jni_call_back;

#[no_mangle]
extern fn contact_send_message(contact: &GlobalRef, message: &GlobalRef) -> *mut GlobalRef {
    let g = contact_send_message0(contact, message).expect("");

    Box::into_raw(Box::new(g))
}

fn contact_send_message0(contact: &GlobalRef, message: &GlobalRef) -> Option<GlobalRef> {
    let contact = contact.clone();
    let message = message.clone();

    jni_call_back(|env| {
        unsafe { crate::contact::send_message_unchecked(env, contact, message) }
    })
}