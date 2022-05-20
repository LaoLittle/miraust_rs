use std::sync::mpsc;

use crate::contact::Contact;
use crate::event::{Event, MessageEvent};
use crate::jni_ffi::jni_callback::call_back;

#[no_mangle]
extern fn message_event_get_subject(event: &Event) -> *mut Contact {
    let contact = message_event_get_subject0(event).expect("Cannot get subject");

    Box::into_raw(Box::new(contact))
}

fn message_event_get_subject0(event: &Event) -> Option<Contact> {
    let global_ref = event.inner.clone();

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { MessageEvent::get_subject_unchecked(global_ref, env) }).unwrap();
    });

    let r = recv.recv().ok()?;

    r.map(|g| { Contact { inner: g } })
}