use std::marker::PhantomPinned;

use libloading::Library;

pub struct RustPlugin {
    _lib: Library,
    func: RustPluginFunc,
    _mark: PhantomPinned,
}

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

    pub fn new(lib: Library, func: RustPluginFunc) -> RustPlugin {
        RustPlugin {
            _lib: lib,
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

/*impl RustPluginDescription {
    pub fn new(id: &str, version: &str) -> RustPluginDescription {
        Self::default(id, version)
    }

    fn default(id: &str, version: &str) -> RustPluginDescription {
        RustPluginDescription {
            author: None,
            id: id.into(),
            name: None,
            version: version.into(),
        }
    }
}*/

/*pub trait ToMirai {
    fn description() -> RustPluginDescription;

    fn on_enable();

    fn on_disable() {
        //
    }

    fn register(&self) -> RustPluginFunc {
        RustPluginFunc { description_fun: Self::description, on_enable_fun: Self::on_enable, on_disable_fun: Self::on_disable }
    }
}*/