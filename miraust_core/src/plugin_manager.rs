use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use crate::event::Event;
use crate::jni_ffi::jni_callback::MIRAI_ENV;

// 统一事件分发器
pub(crate) fn broadcast(env: JNIEnv, _clz: JClass, event: JObject) {
    let global = env.new_global_ref(event).unwrap();
    if MIRAI_ENV.get().unwrap().sender.send(Event { inner: global }).is_err() {
        env.throw_new("java/lang/RuntimeException", "Cannot send").unwrap();
    };
}