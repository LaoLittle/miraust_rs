use std::ffi::c_void;

use jni::{JavaVM, JNIEnv, NativeMethod};
use jni::sys::{jint, JNI_ERR};

use crate::jni_ffi::jni_callback::{CALLBACK_POOL, MIRAI_ENV, MiraiEnv};
use crate::plugin_loader::*;
use crate::plugin_manager::broadcast;

macro_rules! jni_method {
    ( $name:expr, $signature:expr, $fun:tt ) => {{
        jni::NativeMethod {
            name: jni::strings::JNIString::from($name),
            sig: jni::strings::JNIString::from($signature),
            fn_ptr: $fun as *mut c_void,
        }
    }};
}

// jni_onload 实现
#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut c_void) -> jint {
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

    let class_name: &str = "org/laolittle/EventHandler";
    let jni_methods = [
        jni_method!("broadcast", "(Lnet/mamoe/mirai/event/Event;B)V", broadcast)
    ];

    status = register_natives(&jvm, class_name, &jni_methods);
    if status == JNI_ERR { return JNI_ERR; }

    set_callback(jvm);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .on_thread_start(|| {
            let jvm = MIRAI_ENV.get().unwrap().jvm;
            jvm.attach_current_thread_as_daemon().expect("Cannot attach thread to JavaVM");
        })
        .build().unwrap();

    if CALLBACK_POOL.set(runtime).is_err() { status = JNI_ERR };

    status
}

fn register_natives(jvm: &JavaVM, class_name: &str, methods: &[NativeMethod]) -> jint {
    let env: JNIEnv = jvm.get_env().expect("Not from JavaVM");
    let jni_version = env.get_version().expect("Unknown Jvm version");
    let version: jint = jni_version.into();

    let clazz = match env.find_class(class_name) {
        Ok(clazz) => clazz,
        Err(e) => {
            eprintln!("java class not found: {:?}", e);
            return JNI_ERR;
        }
    };
    let result = env.register_native_methods(clazz, methods);

    if result.is_ok() {
        version
    } else {
        JNI_ERR
    }
}

fn set_callback(jvm: JavaVM) {
    let jvm = Box::new(jvm);
    let jvm: &'static JavaVM = Box::leak(jvm);
    let env = jvm.get_env().unwrap();

    fn set_callback_inner(jvm: &'static JavaVM, env: JNIEnv<'static>) -> jni::errors::Result<()> {
        let (sender, _) = tokio::sync::broadcast::channel(32);

        // bot
        let bot_class = env.find_class("net/mamoe/mirai/Bot")?;

        let bot_get_instance = env.get_static_method_id(bot_class, "findInstance", "(J)Lnet/mamoe/mirai/Bot;")?;
        let bot_get_friend = env.get_method_id(bot_class, "getFriend", "(J)Lnet/mamoe/mirai/contact/Friend;")?;
        let bot_get_group = env.get_method_id(bot_class, "getGroup", "(J)Lnet/mamoe/mirai/contact/Group;")?;
        let bot_get_stranger = env.get_method_id(bot_class, "getStranger", "(J)Lnet/mamoe/mirai/contact/Stranger;")?;

        // message event
        let message_event_class = env.find_class("net/mamoe/mirai/event/events/MessageEvent")?;

        let message_event_get_subject = env.get_method_id(message_event_class, "getSubject", "()Lnet/mamoe/mirai/contact/Contact;")?;
        let message_event_get_message = env.get_method_id(message_event_class, "getMessage", "()Lnet/mamoe/mirai/message/data/MessageChain;")?;

        // message
        let message_class = env.find_class("net/mamoe/mirai/message/data/Message")?;

        let message_to_string = env.get_method_id(message_class, "toString", "()Ljava/lang/String;")?;
        let message_content_to_string = env.get_method_id(message_class, "contentToString", "()Ljava/lang/String;")?;

        // plain_text
        let plain_text_class = env.find_class("net/mamoe/mirai/message/data/PlainText")?;

        let new_plain_text = env.get_method_id(plain_text_class, "<init>", "(Ljava/lang/String;)V")?;

        // contact
        let contact_class = env.find_class("net/mamoe/mirai/contact/Contact")?;

        let contact_send_message = env.get_method_id(contact_class, "sendMessage", "(Lnet/mamoe/mirai/message/data/Message;)Lnet/mamoe/mirai/message/MessageReceipt;")?;

        // message_chain builder
        let message_chain_builder_class = env.find_class("net/mamoe/mirai/message/data/MessageChainBuilder")?;

        let new_message_chain_builder = env.get_method_id(message_chain_builder_class, "<init>", "(I)V")?;
        let message_chain_builder_add = env.get_method_id(message_chain_builder_class, "add", "(Lnet/mamoe/mirai/message/data/Message;)Z")?;
        let message_chain_builder_as_message_chain = env.get_method_id(message_chain_builder_class, "asMessageChain", "()Lnet/mamoe/mirai/message/data/MessageChain;")?;


        let plain_text_class = env.new_global_ref(plain_text_class)?;
        let message_chain_builder_class = env.new_global_ref(message_chain_builder_class)?;

        if MIRAI_ENV.set(MiraiEnv {
            jvm,
            sender,
            bot_get_instance,
            bot_get_friend,
            bot_get_group,
            bot_get_stranger,
            message_event_get_subject,
            message_event_get_message,
            message_to_string,
            message_content_to_string,
            plain_text_class,
            new_plain_text,
            contact_send_message,
            message_chain_builder_class,
            new_message_chain_builder,
            message_chain_builder_add,
            message_chain_builder_as_message_chain,
        }).is_err() {
            env.throw_new("java/lang/RuntimeException", "Unable to set mirai_env")?;
        };

        Ok(())
    }

    if let Err(e) = set_callback_inner(jvm, env) {
        env.exception_describe().unwrap();
        env.throw_new("java/lang/RuntimeException", format!("Unable to set callback: {e}")).expect("Unknown reason in set_callback::$set_callback_inner$");
    };
}

/* fn jni_method<F: Fn()>(name: &str, signature: &str, fun: F) -> NativeMethod {
    NativeMethod {
        name: JNIString::from(name),
        sig: JNIString::from(signature),
        fn_ptr: fun as *mut c_void,
    }
} */