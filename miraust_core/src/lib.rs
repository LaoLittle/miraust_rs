#![feature(once_cell)]

use tokio::task::JoinHandle;

use crate::plugin::RustPlugin;

mod test;
mod plugin;
mod plugin_loader;
mod plugin_manager;
mod jni_ffi;
mod bot;
mod contact;
mod api;
mod event;
mod managed;
mod message;

pub type Listener = JoinHandle<()>;