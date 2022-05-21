use std::sync::mpsc;

use jni::objects::GlobalRef;

use crate::contact::Contact;
use crate::event::{Event, MessageEvent};
use crate::jni_ffi::jni_callback::call_back;

#[no_mangle]
extern fn message_event_get_subject(event: &GlobalRef) -> *mut GlobalRef {
    let g = message_event_get_subject0(event).expect("Cannot get subject");

    Box::into_raw(Box::new(g))
}

fn message_event_get_subject0(event: &GlobalRef) -> Option<GlobalRef> {
    let global_ref = event.clone();

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { MessageEvent::get_subject_unchecked(global_ref, env) }).unwrap();
    });

    recv.recv().unwrap()
}

#[no_mangle]
extern fn message_event_get_message(event: &GlobalRef) -> *mut GlobalRef {
    let g = message_event_get_message0(event).expect("Cannot get subject");

    Box::into_raw(Box::new(g))
}

fn message_event_get_message0(event: &GlobalRef) -> Option<GlobalRef> {
    let global_ref = event.clone();

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { MessageEvent::get_message_unchecked(global_ref, env) }).unwrap();
    });

    recv.recv().unwrap()
}