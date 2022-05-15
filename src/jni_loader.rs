use std::ffi::c_void;
use std::sync::Mutex;

use jni::{AttachGuard, JavaVM, JNIEnv, NativeMethod};
use jni::objects::GlobalRef;
use jni::sys::{jint, JNI_ERR};
use lazy_static::lazy_static;

use crate::jni_callback::{MIRAI_ENV, MiraiEnv};
use crate::plugin_loader::*;

macro_rules! jni_method {
    ( $name:expr, $signature:expr, $fun:tt ) => {{
        jni::NativeMethod {
            name: jni::strings::JNIString::from($name),
            sig: jni::strings::JNIString::from($signature),
            fn_ptr: $fun as *mut c_void,
        }
    }};
}

lazy_static! {
    // jvm
    static ref JVM_GLOBAL: Mutex<Option<JavaVM>> = Mutex::new(None);
    //callback
    static ref JNI_CALLBACK: Mutex<Option<GlobalRef>> = Mutex::new(None);
}

// jni_onload 实现
pub(crate) fn jni_onload(jvm: JavaVM, _reserved: *mut c_void) -> jint {
    let mut status: jint;
    // register to class RustPluginLoader
    let plugin_loader = "org/laolittle/loader/RustPluginLoader";

    let loader_methods = [
        jni_method!("loadPlugin", "(Ljava/lang/String;)J", load_plugin),
        jni_method!("unloadPlugin", "(J)V", unload_plugin),
        jni_method!("getPluginDescription", "(J)[Ljava/lang/String;", get_plugin_description),
        jni_method!("enablePlugin", "(J)V", enable_plugin),
        jni_method!("disablePlugin", "(J)V", disable_plugin),
    ];

    status = register_natives(&jvm, plugin_loader, &loader_methods);
    if status == JNI_ERR { return JNI_ERR; }

    // for test now
    let class_name: &str = "org/laolittle/EventHandler";
    let jni_methods = [
        //jni_method!("broadcast", "(Ljava/lang/String;)V", broadcast)
    ];

    status = register_natives(&jvm, class_name, &jni_methods);
    if status == JNI_ERR { return JNI_ERR; }

    set_callback(jvm);

    status
}

fn register_natives(jvm: &JavaVM, class_name: &str, methods: &[NativeMethod]) -> jint {
    let env: JNIEnv = jvm.get_env().unwrap();
    let jni_version = env.get_version().unwrap();
    let version: jint = jni_version.into();

    let clazz = match env.find_class(class_name) {
        Ok(clazz) => clazz,
        Err(e) => {
            eprintln!("java class not found : {:?}", e);
            return JNI_ERR;
        }
    };
    let result = env.register_native_methods(clazz, &methods);

    if result.is_ok() {
        version
    } else {
        JNI_ERR
    }
}

fn set_callback(jvm: JavaVM) {
    let jvm = Box::new(jvm);
    let jvm: &'static JavaVM = Box::leak(jvm);
    let env: AttachGuard = jvm.attach_current_thread().unwrap();

    let bot_get_instance = env.get_static_method_id("net/mamoe/mirai/Bot", "getInstanceOrNull", "(J)Lnet/mamoe/mirai/Bot;").unwrap();
    let bot_get_friend = env.get_method_id("net/mamoe/mirai/Bot", "getFriend", "(J)Lnet/mamoe/mirai/contact/Friend;").unwrap();
    if let Err(mirai) = MIRAI_ENV.set(MiraiEnv {
        jvm,
        env,
        bot_get_instance,
        bot_get_friend,
    }) {
        mirai.env.throw_new("java/lang/RuntimeException", "").unwrap();
    };
}

/* fn jni_method<F: Fn()>(name: &str, signature: &str, fun: F) -> NativeMethod {
    NativeMethod {
        name: JNIString::from(name),
        sig: JNIString::from(signature),
        fn_ptr: fun as *mut c_void,
    }
} */