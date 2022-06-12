use std::lazy::SyncOnceCell;
use std::sync::mpsc;

use jni::{JavaVM, JNIEnv};
use jni::objects::{GlobalRef, JMethodID, JStaticMethodID};

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
    pub(crate) plain_text_class: GlobalRef,
    pub(crate) new_plain_text: JMethodID<'static>,
    pub(crate) contact_send_message: JMethodID<'static>,
    pub(crate) message_chain_builder_class: GlobalRef,
    pub(crate) new_message_chain_builder: JMethodID<'static>,
    pub(crate) message_chain_builder_add: JMethodID<'static>,
    pub(crate) message_chain_builder_as_message_chain: JMethodID<'static>,
}

pub(crate) fn spawn_call_back<F>(fun: F)
    where F: FnOnce(JNIEnv) + Send + 'static
{
    let runtime = CALLBACK_POOL.get().unwrap();
    let mirai = MIRAI_ENV.get().unwrap();

    runtime.spawn_blocking(|| {
        let env = mirai.jvm.get_env().unwrap();
        fun(env);
    });
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