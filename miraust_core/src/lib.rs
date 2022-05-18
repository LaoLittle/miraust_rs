#![feature(once_cell)]

use crate::plugin::RustPlugin;

mod test;
pub mod plugin;
mod plugin_loader;
mod plugin_manager;
mod jni_ffi;
pub mod bot;
pub mod contact;
mod api;