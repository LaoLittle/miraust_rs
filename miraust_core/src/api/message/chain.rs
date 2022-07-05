use jni::objects::GlobalRef;
use crate::jni_ffi::jni_callback::jni_call_back;

#[no_mangle]
extern fn message_chain_iterator(chain: &GlobalRef) -> *mut GlobalRef {
    let g = message_chain_iterator0(chain).unwrap();

    Box::into_raw(Box::new(g))
}

fn message_chain_iterator0(chain: &GlobalRef) -> Option<GlobalRef> {
    let chain = chain.clone();

    jni_call_back(|env| {
        unsafe { crate::message::chain::iterator::iterator_unchecked(env, chain) }
    })
}

#[no_mangle]
extern fn message_chain_iterator_has_next(iter: &GlobalRef) -> bool {
    message_chain_iterator_has_next0(iter)
}

fn message_chain_iterator_has_next0(iter: &GlobalRef) -> bool {
    let iter = iter.clone();

    jni_call_back(move |env| {
        Some(unsafe { crate::message::chain::iterator::iterator_has_next(env,iter) })
    }).unwrap()
}

#[no_mangle]
extern fn message_chain_iterator_next(iter: &GlobalRef) -> *mut GlobalRef {
    let g = message_chain_iterator_next0(iter).unwrap();

    Box::into_raw(Box::new(g))
}

fn message_chain_iterator_next0(iter: &GlobalRef) -> Option<GlobalRef> {
    let iter = iter.clone();

    jni_call_back(move |env| {
        unsafe { crate::message::chain::iterator::iterator_next(env, iter) }
    })
}