use jni::JNIEnv;
use jni::objects::GlobalRef;
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::MIRAI_ENV;
use crate::managed::Managed;

#[derive(Clone)]
pub(crate) struct Event {
    pub(crate) inner: GlobalRef,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct EventManaged {
    pub(crate) inner: Managed,
    pub(crate) event_type: u8,
}

pub(crate) struct MessageEvent {
    pub(crate) inner: Event,
}

impl MessageEvent {
    pub unsafe fn get_subject_unchecked(global_ref: GlobalRef, env: JNIEnv) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.message_event_get_subject,
            JavaType::Object("net/mamoe/mirai/contact/Contact".to_string()),
            &[],
        ) {
            Some(env.new_global_ref(value.l().unwrap()).ok()?)
        } else { None }
    }

    pub unsafe fn get_message_unchecked(global_ref: GlobalRef, env: JNIEnv) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.message_event_get_message,
            JavaType::Object("net/mamoe/mirai/message/data/MessageChain".to_string()),
            &[],
        ) {
            Some(env.new_global_ref(value.l().unwrap()).ok()?)
        } else { None }
    }
}