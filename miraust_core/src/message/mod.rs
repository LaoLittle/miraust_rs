use jni::JNIEnv;
use jni::objects::GlobalRef;
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::MIRAI_ENV;

pub unsafe fn to_string_unchecked(env: JNIEnv, global_ref: GlobalRef) -> Option<String> {
    let mirai = MIRAI_ENV.get()?;

    if let Ok(value) = env.call_method_unchecked(
        global_ref.as_obj(),
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

pub unsafe fn content_to_string_unchecked(env: JNIEnv, global_ref: GlobalRef) -> Option<String> {
    let mirai = MIRAI_ENV.get()?;

    if let Ok(value) = env.call_method_unchecked(
        global_ref.as_obj(),
        mirai.message_content_to_string,
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