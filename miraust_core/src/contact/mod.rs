use jni::JNIEnv;
use jni::objects::{GlobalRef, JValue};
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::MIRAI_ENV;

pub unsafe fn send_message_unchecked(env: JNIEnv, contact: GlobalRef, message: GlobalRef) -> Option<GlobalRef> {
    let mirai = MIRAI_ENV.get()?;

    if let Ok(val) = env.call_method_unchecked(
        contact.as_obj(),
        mirai.contact_send_message,
        JavaType::Object(String::new() /*"net/mamoe/mirai/message/MessageReceipt".to_string()*/),
        &[JValue::Object(message.as_obj())],
    ) {
        let obj = val.l().ok()?;
        if obj.is_null() { return None; }

        Some(env.new_global_ref(obj).ok()?)
    } else {
        None
    }
}