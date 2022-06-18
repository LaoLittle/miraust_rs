use std::marker::PhantomPinned;

use libloading::Library;

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
    description_fun: fn() -> RustPluginDescription,
    on_enable_fun: fn(*const ()),
    on_disable_fun: fn(*const ()),
    plugin_unload_fun: fn(*mut ()),
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

#[derive(Debug)]
#[repr(C)]
pub struct RustPluginDescription {
    pub author: Option<String>,
    pub id: String,
    pub name: Option<String>,
    pub version: String,
}