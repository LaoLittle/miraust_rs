use std::lazy::SyncOnceCell;

use jni::{JavaVM, JNIEnv};
use jni::objects::{JMethodID, JStaticMethodID};

pub(crate) static MIRAI_ENV: SyncOnceCell<MiraiEnv> = SyncOnceCell::new();
pub(crate) static CALLBACK_POOL: SyncOnceCell<tokio::runtime::Runtime> = SyncOnceCell::new();


pub(crate) struct MiraiEnv {
    pub(crate) jvm: &'static JavaVM,
    pub(crate) sender: tokio::sync::broadcast::Sender<()>,
    pub(crate) bot_get_instance: JStaticMethodID<'static>,
    pub(crate) bot_get_friend: JMethodID<'static>,
    pub(crate) bot_get_group: JMethodID<'static>,
    pub(crate) bot_get_stranger: JMethodID<'static>
}

pub(crate) fn call_back<F>(fun: F) -> tokio::task::JoinHandle<()>
    where F: FnOnce(JNIEnv) + Send + 'static
{
    let runtime = CALLBACK_POOL.get().unwrap();

    runtime.spawn(async {
        let mirai = MIRAI_ENV.get().unwrap();
        let env = mirai.jvm.get_env().unwrap();
        fun(env)
    })
}

impl MiraiEnv {}

unsafe impl Send for MiraiEnv {}

unsafe impl Sync for MiraiEnv {}