use jni::objects::GlobalRef;

use crate::jni_ffi::jni_callback::jni_call_back;
use crate::RawString;

mod chain;

#[no_mangle]
extern fn message_to_string(message: &GlobalRef) -> RawString {
    message_to_string0(message).expect("Error on Message.toString()").into_raw_parts().into()
}

fn message_to_string0(message: &GlobalRef) -> Option<String> {
    let global_ref = message.clone();

    jni_call_back(|env| {
        unsafe { crate::message::to_string_unchecked(env, global_ref) }
    })
}

#[no_mangle]
extern fn message_content_to_string(message: &GlobalRef) -> RawString {
    message_content_to_string0(message).expect("Error on Message.contentToString()").into()
}

fn message_content_to_string0(message: &GlobalRef) -> Option<String> {
    let global_ref = message.clone();

    jni_call_back(|env| {
        unsafe { crate::message::content_to_string_unchecked(env, global_ref) }
    })
}

mod single {
    mod plain_text {
        use jni::objects::GlobalRef;

        use crate::jni_ffi::jni_callback::jni_call_back;
        use crate::RawString;

        #[no_mangle]
        extern fn new_plain_text(str: RawString) -> *mut GlobalRef {
            let g = new_plain_text0(str).unwrap();

            Box::into_raw(Box::new(g))
        }

        fn new_plain_text0(str: RawString) -> Option<GlobalRef> {
            jni_call_back(move |env| crate::message::single::plain_text::new_unchecked(env, str).map_err(|e| panic!("{e}")).ok())
        }
    }
}

mod chain_builder {
    use jni::objects::GlobalRef;

    use crate::jni_ffi::jni_callback::jni_call_back;

    #[no_mangle]
    extern fn new_message_chain_builder(initial_size: i32) -> *mut GlobalRef {
        let g = new_message_chain_builder0(initial_size).map_err(|e| panic!("Error on new MessageChainBuilder(): {e}")).unwrap();
        Box::into_raw(Box::new(g))
    }

    fn new_message_chain_builder0(initial_size: i32) -> jni::errors::Result<GlobalRef> {
        jni_call_back(move |env| {
            let ss = Some(unsafe { crate::message::chain::builder::new_unchecked(env, initial_size) });
            env.exception_describe().unwrap();
            ss
        }).unwrap()
    }

    #[no_mangle]
    extern fn message_chain_builder_add(chain: &GlobalRef, message: &GlobalRef) -> bool {
        message_chain_builder_add0(chain, message)
    }

    fn message_chain_builder_add0(chain: &GlobalRef, message: &GlobalRef) -> bool {
        let (chain, message) = (chain.clone(), message.clone());

        jni_call_back(move |env| Some(unsafe { crate::message::chain::builder::add_unchecked(env, chain, message) })).unwrap()
    }

    #[no_mangle]
    extern fn message_chain_builder_as_message_chain(chain: &GlobalRef) -> *mut GlobalRef {
        let g = message_chain_builder_as_message_chain0(chain).unwrap();

        Box::into_raw(Box::new(g))
    }

    fn message_chain_builder_as_message_chain0(chain: &GlobalRef) -> Option<GlobalRef> {
        let chain = chain.clone();

        jni_call_back(move |env| Some(unsafe { crate::message::chain::builder::as_message_chain_unchecked(env, chain) })).unwrap()
    }
}