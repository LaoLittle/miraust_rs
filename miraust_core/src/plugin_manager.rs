use jni::JNIEnv;
use jni::objects::{JClass, JObject};

use crate::event::Event;
use crate::jni_ffi::jni_callback::MIRAI_ENV;

// 统一事件分发器
pub(crate) fn broadcast(env: JNIEnv, _clz: JClass, event: JObject, t: u8) {
    let global = env.new_global_ref(event).unwrap();
    // let event = Event { inner: global };

    if MIRAI_ENV.get().unwrap().sender.send((global, t)).is_err() {
        //
    };
}