use std::marker::PhantomPinned;

use libloading::Library;

use crate::RawString;

pub struct RustPlugin {
    _lib: Library,
    interface: RustPluginInterface,
    _mark: PhantomPinned,
}

impl RustPlugin {
    fn raw_instance(&self) -> *const () {
        self.raw_instance_mut()
    }

    fn raw_instance_mut(&self) -> *mut () {
        self.interface.instance
    }
}

#[repr(C)]
pub struct RustPluginInterface {
    instance: *mut (),
    description_fun: extern fn() -> RustPluginDescription,
    on_enable_fun: extern fn(*const ()),
    on_disable_fun: extern fn(*const ()),
    plugin_unload_fun: extern fn(*mut ()),
}

impl RustPlugin {
    pub fn description(&self) -> RustPluginDescription {
        let fun = self.interface.description_fun;
        fun()
    }

    pub fn enable(&self) {
        let fun = self.interface.on_enable_fun;
        fun(self.raw_instance())
    }

    pub fn disable(&self) {
        let fun = self.interface.on_disable_fun;
        fun(self.raw_instance())
    }

    pub fn unload(&mut self) {
        let fun = self.interface.plugin_unload_fun;
        fun(self.raw_instance_mut())
    }

    pub fn new(_lib: Library, interface: RustPluginInterface) -> RustPlugin {
        RustPlugin {
            _lib,
            interface,
            _mark: PhantomPinned,
        }
    }
}

#[repr(C)]
pub struct RustPluginDescription {
    author: RawString,
    id: RawString,
    name: RawString,
    version: RawString,
}

impl RustPluginDescription {
    pub fn author(&self) -> &str {
        self.author.get_str().unwrap_or("")
    }

    pub fn id(&self) -> &str {
        self.id.get_str().unwrap_or("")
    }

    pub fn name(&self) -> &str {
        self.name.get_str().unwrap_or_else(|| self.id())
    }

    pub fn version(&self) -> &str {
        self.version.get_str().unwrap_or("0.0.1")
    }
}

impl Drop for RustPluginDescription {
    fn drop(&mut self) {
        let _ = self.author.string();
        let _ = self.id.string();
        let _ = self.name.string();
        let _ = self.version.string();
    }
}