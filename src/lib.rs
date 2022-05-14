use std::ffi::c_void;

use jni::{JavaVM, JNIEnv, sys::{jint, jobject, jstring}};

use crate::jni_loader::jni_onload;
use crate::plugin::RustPlugin;

pub mod event;
mod test;
pub mod plugin;
mod plugin_loader;
mod plugin_manager;
mod jni_loader;

fn broadcast(env: JNIEnv, obj: jobject, content: jstring) {
    todo!()
}

#[no_mangle]
#[allow(non_snake_case)]
fn JNI_OnLoad(jvm: JavaVM, _reserved: *mut c_void) -> jint {
    jni_onload(jvm, _reserved)
}