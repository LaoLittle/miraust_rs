use std::path::Path;
use std::ptr::null;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jobjectArray, jsize};
use libloading::Library;

use crate::plugin::RustPluginFunc;
use crate::RustPlugin;

pub(crate) fn load_plugin(env: JNIEnv, _obj: JObject, str: JString) -> *const RustPlugin {
    let str = env.get_string(str).unwrap();
    let str = str.to_str().unwrap();
    let path = Path::new(str);

    let lib = unsafe { Library::new(path) };
    match lib {
        Ok(lib) => {
            let fun_on_load = unsafe { lib.get::<fn() -> RustPluginFunc>(b"on_load").unwrap() };

            let func: RustPluginFunc = fun_on_load();
            let plugin_instance = RustPlugin::new(lib, func);

            let plugin_box = Box::new(plugin_instance);
            Box::into_raw(plugin_box)
        }
        Err(e) => {
            env.throw_new("java/lang/RuntimeException", format!("Cannot load Rust plugin: {}", e)).unwrap();
            null()
        }
    }
}

pub(crate) fn unload_plugin(_env: JNIEnv, _obj: JObject, plugin: *mut RustPlugin) {
    unsafe { Box::from_raw(plugin) };
}

pub(crate) fn get_plugin_description(env: JNIEnv, _obj: JObject, plugin: *const RustPlugin) -> jobjectArray {
    let plugin = unsafe { &*plugin };

    let string_array = env.new_object_array(
        4,
        "java/lang/String",
        env.new_string("").unwrap(),
    ).unwrap();

    let desc = plugin.description();
    let id = desc.id.as_str();
    let name = if let Some(ref name) = desc.name {
        name.as_str()
    } else { id };

    let author = if let Some(ref author) = desc.author {
        author.as_str()
    } else { "" };
    let version = desc.version.as_str();

    {
        for (i, s) in [id, name, author, version].into_iter().enumerate() {
            env.set_object_array_element(string_array, i as jsize, env.new_string(s).unwrap()).unwrap();
        };
    }

    string_array
}

pub(crate) fn enable_plugin(_env: JNIEnv, _obj: JObject, plugin: *const RustPlugin) {
    let plugin = unsafe { &*plugin };

    plugin.enable();
}

pub(crate) fn disable_plugin(_env: JNIEnv, _obj: JObject, plugin: *const RustPlugin) {
    let plugin = unsafe { &*plugin };

    plugin.disable();
}