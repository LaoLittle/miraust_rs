use jni::JNIEnv;
use jni::objects::GlobalRef;
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::MIRAI_ENV;

pub mod chain;
pub mod single;

pub unsafe fn to_string_unchecked(env: JNIEnv, message: GlobalRef) -> Option<String> {
    let mirai = MIRAI_ENV.get()?;

    if let Ok(value) = env.call_method_unchecked(
        message.as_obj(),
        mirai.message_to_string,
        JavaType::Object("java/lang/String".to_string()),
        &[],
    ) {
        let obj = value.l().ok()?;
        if obj.is_null() { return None; }

        let jstr = env.get_string(obj.into()).ok()?;
        Some(jstr.to_str().expect("Unknown").to_string())
    } else {
        None
    }
}

pub unsafe fn content_to_string_unchecked(env: JNIEnv, message: GlobalRef) -> Option<String> {
    let mirai = MIRAI_ENV.get()?;

    if let Ok(value) = env.call_method_unchecked(
        message.as_obj(),
        mirai.message_content_to_string,
        JavaType::Object(String::new() /*"java/lang/String".to_string()*/),
        &[],
    ) {
        let obj = value.l().ok()?;
        if obj.is_null() { return None; }

        let jstr = env.get_string(obj.into()).ok()?;
        Some(jstr.to_str().expect("Unknown").to_string())
    } else {
        None
    }
}