use std::lazy::SyncOnceCell;

use jni::{AttachGuard, JavaVM};
use jni::objects::{JMethodID, JStaticMethodID};

pub(crate) static MIRAI_ENV: SyncOnceCell<MiraiEnv> = SyncOnceCell::new();

pub(crate) struct MiraiEnv {
    pub(crate) jvm: &'static JavaVM,
    pub(crate) env: AttachGuard<'static>,
    pub(crate) bot_get_instance: JStaticMethodID<'static>,
    pub(crate) bot_get_friend: JMethodID<'static>,
}

impl MiraiEnv {}

unsafe impl Send for MiraiEnv {}

unsafe impl Sync for MiraiEnv {}