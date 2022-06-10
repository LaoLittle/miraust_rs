use std::marker::PhantomPinned;

use libloading::Library;

pub struct RustPlugin {
    _lib: Library,
    func: RustPluginFunc,
    _mark: PhantomPinned,
}

#[repr(C)]
pub struct RustPluginFunc {
    description_fun: fn() -> RustPluginDescription,
    on_enable_fun: fn(),
    on_disable_fun: fn(),
}

impl RustPlugin {
    pub fn description(&self) -> RustPluginDescription {
        let fun = self.func.description_fun;
        fun()
    }

    pub fn enable(&self) {
        let fun = self.func.on_enable_fun;
        fun()
    }

    pub fn disable(&self) {
        let fun = self.func.on_disable_fun;
        fun()
    }

    pub fn new(_lib: Library, func: RustPluginFunc) -> RustPlugin {
        RustPlugin {
            _lib,
            func,
            _mark: PhantomPinned,
        }
    }
}

#[derive(Debug)]
pub struct RustPluginDescription {
    pub author: Option<String>,
    pub id: String,
    pub name: Option<String>,
    pub version: String,
}