use std::fmt::{Display, Formatter};
use std::sync::mpsc;

use jni::JNIEnv;
use jni::objects::{GlobalRef, JValue};
use jni::signature::JavaType;

use crate::contact::friend::Friend;
use crate::jni_ffi::jni_callback::{CALLBACK_POOL, MIRAI_ENV};

pub struct Bot {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl<'a> Bot {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn instances() {
        // todo
    }

    pub fn get_instance(id: i64) -> Option<Bot> {
        let (send, recv) = mpsc::channel();
        CALLBACK_POOL.get().unwrap().execute(move |env| {
            send.send(unsafe { Bot::get_instance_unchecked(env, id) }).unwrap();
        }).ok()?;

        let r = recv.recv().ok()?;

        r.map(|global_ref| Bot { id, inner: global_ref })
    }

    pub fn get_friend(&self, id: i64) -> Option<Friend> {
        let global_ref = self.inner.clone();
        let (send, recv) = mpsc::channel();
        CALLBACK_POOL.get().unwrap().execute(move |env| {
            send.send(unsafe { Bot::get_friend_unchecked(global_ref, env, id) }).unwrap();
        }).ok()?;

        let r = recv.recv().ok()?;

        r.map(|global_ref| Friend { id, inner: global_ref })
    }

    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn get_instance_unchecked(env: &'a JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(bot) = env.call_static_method_unchecked(
            "net/mamoe/mirai/Bot",
            mirai.bot_get_instance,
            JavaType::Object("net/mamoe/mirai/Bot".to_string()),
            &[JValue::Long(id)],
        ) {
            Some(env.new_global_ref(bot.l().ok()?).ok()?)
        } else {
            None
        }
    }

    /// # Safety
    /// This function will not attach thread to jvm
    pub unsafe fn get_friend_unchecked(global_ref: GlobalRef, env: &'a JNIEnv, id: i64) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get()?;

        if let Ok(friend) = env.call_method_unchecked(
            global_ref.as_obj(),
            mirai.bot_get_friend,
            JavaType::Object("net/mamoe/mirai/contact/Friend".to_string()),
            &[JValue::Long(id)],
        ) {
            Some(env.new_global_ref(friend.l().ok()?).ok()?)
        } else {
            None
        }
    }
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bot: {}", self.id)
    }
}