use jni::JNIEnv;
use jni::objects::GlobalRef;
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::MIRAI_ENV;

pub struct Message {
    inner: GlobalRef,
}

impl Message {
    pub unsafe fn to_string_unchecked(global_ref: GlobalRef, env: JNIEnv) -> Option<String> {
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
}