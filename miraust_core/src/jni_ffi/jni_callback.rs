use std::lazy::SyncOnceCell;

use jni::{JavaVM, JNIEnv};
use jni::objects::{GlobalRef, JMethodID, JStaticMethodID};

use crate::event::Event;

pub(crate) static MIRAI_ENV: SyncOnceCell<MiraiEnv> = SyncOnceCell::new();
pub(crate) static CALLBACK_POOL: SyncOnceCell<tokio::runtime::Runtime> = SyncOnceCell::new();


pub(crate) struct MiraiEnv {
    pub(crate) jvm: &'static JavaVM,
    pub(crate) sender: tokio::sync::broadcast::Sender<(GlobalRef, u8)>,
    pub(crate) bot_get_instance: JStaticMethodID<'static>,
    pub(crate) bot_get_friend: JMethodID<'static>,
    pub(crate) bot_get_group: JMethodID<'static>,
    pub(crate) bot_get_stranger: JMethodID<'static>,
    pub(crate) message_event_get_subject: JMethodID<'static>,
    pub(crate) message_event_get_message: JMethodID<'static>,
    pub(crate) message_to_string: JMethodID<'static>,
}

pub(crate) fn call_back<F>(fun: F) -> tokio::task::JoinHandle<()>
    where F: FnOnce(JNIEnv) + Send + 'static
{
    let runtime = CALLBACK_POOL.get().unwrap();
    let mirai = MIRAI_ENV.get().unwrap();

    runtime.spawn(async {
        let env = mirai.jvm.get_env().unwrap();
        fun(env)
    })
}

impl MiraiEnv {}

unsafe impl Send for MiraiEnv {}

unsafe impl Sync for MiraiEnv {}