use std::fmt::{Display, Formatter};
use jni::objects::{JObject, JValue};
use jni::signature::JavaType;
use crate::contact::friend::Friend;

use crate::jni_callback::MIRAI_ENV;

#[derive(Debug)]
pub struct Bot<'a> {
    pub(crate) id: i64,
    pub(crate) inner: JObject<'a>,
}

impl<'a> Bot<'a> {
    pub fn instances() {
        // todo
    }

    pub fn get_instance(id: i64) -> Option<Bot<'a>> {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(bot) = mirai.env.call_static_method_unchecked(
            "net/mamoe/mirai/Bot$Companion",
            mirai.bot_get_instance,
            JavaType::Object("net/mamoe/mirai/Bot".to_string()),
            &[JValue::Long(id)],
        ) {
            Some(Bot { id, inner: bot.l().unwrap() })
        } else {
            None
        }
    }

    pub fn get_friend(&self, id: i64) -> Option<Friend<'a>> {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(friend) = mirai.env.call_method_unchecked(
            self.inner,
            mirai.bot_get_friend,
            JavaType::Object("net/mamoe/mirai/contact/Friend".to_string()),
            &[JValue::Long(id)],
        ) {
            Some(Friend { inner: friend.l().unwrap() })
        } else {
            None
        }
    }
}

impl<'a> Display for Bot<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bot: {}", self.id)
    }
}