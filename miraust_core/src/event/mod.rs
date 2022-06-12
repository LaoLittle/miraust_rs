pub mod message_event {
    use jni::{JNIEnv, objects::GlobalRef, signature::JavaType};

    use crate::jni_ffi::jni_callback::MIRAI_ENV;

    pub unsafe fn get_subject_unchecked(env: JNIEnv, message_event: GlobalRef) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(val) = env.call_method_unchecked(
            message_event.as_obj(),
            mirai.message_event_get_subject,
            JavaType::Object("net/mamoe/mirai/contact/Contact".to_string()),
            &[],
        ) {
            Some(env.new_global_ref(val.l().unwrap()).ok()?)
        } else {
            None
        }
    }

    pub unsafe fn get_message_unchecked(env: JNIEnv, message_event: GlobalRef) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(val) = env.call_method_unchecked(
            message_event.as_obj(),
            mirai.message_event_get_message,
            JavaType::Object(String::new() /*"net/mamoe/mirai/message/data/MessageChain".to_string()*/),
            &[],
        ) {
            Some(env.new_global_ref(val.l().unwrap()).ok()?)
        } else {
            None
        }
    }
}
