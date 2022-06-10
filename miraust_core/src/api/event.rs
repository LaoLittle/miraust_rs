use jni::objects::GlobalRef;

use crate::jni_ffi::jni_callback::jni_call_back;

#[no_mangle]
extern fn message_event_get_subject(event: &GlobalRef) -> *mut GlobalRef {
    let g = message_event_get_subject0(event).expect("Cannot get subject");

    Box::into_raw(Box::new(g))
}

fn message_event_get_subject0(event: &GlobalRef) -> Option<GlobalRef> {
    let global_ref = event.clone();

    jni_call_back(move |env| {
        unsafe { crate::event::message_event::get_subject_unchecked(env, global_ref) }
    })
}

#[no_mangle]
extern fn message_event_get_message(event: &GlobalRef) -> *mut GlobalRef {
    let g = message_event_get_message0(event).expect("Cannot get subject");

    Box::into_raw(Box::new(g))
}

fn message_event_get_message0(event: &GlobalRef) -> Option<GlobalRef> {
    let global_ref = event.clone();

    jni_call_back(move |env| {
        unsafe { crate::event::message_event::get_message_unchecked(env, global_ref) }
    })
}