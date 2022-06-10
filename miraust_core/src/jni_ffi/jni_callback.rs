use std::lazy::SyncOnceCell;
use std::sync::mpsc;

use jni::{JavaVM, JNIEnv};
use jni::objects::{GlobalRef, JMethodID, JStaticMethodID};
use tokio::task::JoinHandle;

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
    pub(crate) message_content_to_string: JMethodID<'static>,
    pub(crate) contact_send_message: JMethodID<'static>,
}

pub(crate) fn spawn_call_back<F>(fun: F) -> JoinHandle<()>
    where F: FnOnce(JNIEnv) + Send + 'static
{
    let runtime = CALLBACK_POOL.get().unwrap();
    let mirai = MIRAI_ENV.get().unwrap();

    runtime.spawn_blocking(move || {
        let env = mirai.jvm.get_env().unwrap();
        fun(env);
    })
}

pub(crate) fn jni_call_back<F, T>(fun: F) -> Option<T>
    where F: FnOnce(JNIEnv) -> Option<T> + Send + 'static,
          T: Send + 'static
{
    let (send, recv) = mpsc::channel();
    spawn_call_back(move |env| {
        send.send(fun(env)).expect("Unable to send");
    });

    recv.recv().ok()?
}

impl MiraiEnv {}

unsafe impl Send for MiraiEnv {}

unsafe impl Sync for MiraiEnv {}