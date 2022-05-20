use std::mem;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::JavaType;

use crate::jni_ffi::jni_callback::{MIRAI_ENV};

pub struct Bot {
    pub(crate) inner: GlobalRef,
}

impl Bot {
    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn find_instance_unchecked(env: JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_static_method_unchecked(
            "net/mamoe/mirai/Bot",
            mirai.bot_get_instance,
            JavaType::Object("net/mamoe/mirai/Bot".to_string()),
            &[JValue::Long(id)],
        ) {
            let obj = value.l().ok()?;
            if obj.is_null() { return None }

            Some(env.new_global_ref(obj).ok()?)
        } else {
            None
        }
    }

    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn get_friend_unchecked(global_ref: GlobalRef, env: JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.bot_get_friend,
            JavaType::Object("net/mamoe/mirai/contact/Friend".to_string()),
            &[JValue::Long(id)],
        ) {
            let obj = value.l().ok()?;
            if obj.is_null() { return None }

            Some(env.new_global_ref(obj).ok()?)
        } else {
            None
        }
    }

    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn get_group_unchecked(global_ref: GlobalRef, env: JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.bot_get_group,
            JavaType::Object("net/mamoe/mirai/contact/Group".to_string()),
            &[JValue::Long(id)],
        ) {
            let obj = value.l().ok()?;
            if obj.is_null() { return None }

            Some(env.new_global_ref(obj).ok()?)
        } else {
            None
        }
    }

    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn get_stranger_unchecked(global_ref: GlobalRef, env: JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(value) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.bot_get_stranger,
            JavaType::Object("net/mamoe/mirai/contact/Stranger".to_string()),
            &[JValue::Long(id)],
        ) {
            let obj = value.l().ok()?;
            if obj.is_null() { return None }

            Some(env.new_global_ref(obj).ok()?)
        } else {
            None
        }
    }
}

trait CanBeNull {
    fn is_null(&self) -> bool;
}

impl<'a> CanBeNull for JObject<'a> {
    fn is_null(&self) -> bool {
        unsafe { *mem::transmute::<&JObject, &usize>(self) == 0 }
    }
}