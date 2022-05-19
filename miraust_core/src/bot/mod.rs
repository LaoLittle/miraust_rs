use std::fmt::{Display, Formatter};
use std::mem;

use std::sync::mpsc;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::JavaType;

use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;
use crate::jni_ffi::jni_callback::{call_back, MIRAI_ENV};

pub struct Bot {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl<'a> Bot {
    pub const fn id(&self) -> u64 {
        self.id as u64
    }

    pub fn instances() {
        // todo
    }

    pub fn find_instance(id: u64) -> Option<Bot> {
        let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

        let (send, recv) = mpsc::channel();
        call_back(move |env| {
            send.send(unsafe { Bot::find_instance_unchecked(env, id) }).unwrap();
        });

        let r = recv.recv().ok()?;

        r.map(|global_ref| Bot { id, inner: global_ref })
    }

    pub fn get_friend(&self, id: u64) -> Option<Friend> {
        let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

        let global_ref = self.inner.clone();
        let (send, recv) = mpsc::channel();
        call_back(move |env| {
            send.send(unsafe { Bot::get_friend_unchecked(global_ref, env, id) }).unwrap();
        });

        let r = recv.recv().ok()?;

        r.map(|global_ref| Friend { id, inner: global_ref })
    }

    pub fn get_group(&self, id: u64) -> Option<Group> {
        let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

        let global_ref = self.inner.clone();
        let (send, recv) = mpsc::channel();
        call_back(move |env| {
            send.send(unsafe { Bot::get_group_unchecked(global_ref, env, id) }).unwrap();
        });

        let r = recv.recv().ok()?;

        r.map(|global_ref| Group { id, inner: global_ref })
    }

    pub fn get_stranger(&self, id: u64) -> Option<Stranger> {
        let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

        let global_ref = self.inner.clone();
        let (send, recv) = mpsc::channel();
        call_back(move |env| {
            send.send(unsafe { Bot::get_stranger_unchecked(global_ref, env, id) }).unwrap();
        });

        let r = recv.recv().ok()?;

        r.map(|global_ref| Stranger { id, inner: global_ref })
    }

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

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bot: {}", self.id)
    }
}