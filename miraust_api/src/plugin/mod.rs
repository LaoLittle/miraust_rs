use std::mem;

use crate::RawString;

impl<P: Plugin> From<P> for RustPluginInterface {
    fn from(plugin: P) -> Self {
        let (description_fun,
            on_enable_fun,
            on_disable_fun,
            plugin_unload_fun) = plugin_interface::<P>();

        let b = Box::into_raw(Box::new(plugin));
        Self {
            instance: b as *mut (),
            description_fun,
            on_enable_fun,
            on_disable_fun,
            plugin_unload_fun,
        }
    }
}

#[repr(C)]
pub struct RustPluginInterface {
    instance: *mut (),
    description_fun: extern fn() -> RustPluginDescFFI,
    on_enable_fun: extern fn(*const ()),
    on_disable_fun: extern fn(*const ()),
    plugin_unload_fun: extern fn(*mut ()),
}

#[derive(Debug)]
pub struct RustPluginDescription {
    pub author: Option<String>,
    pub id: String,
    pub name: Option<String>,
    pub version: String,
}

#[repr(C)]
struct RustPluginDescFFI {
    author: RawString,
    id: RawString,
    name: RawString,
    version: RawString,
}

impl From<RustPluginDescription> for RustPluginDescFFI {
    fn from(desc: RustPluginDescription) -> Self {
        Self {
            author: if let Some(s) = desc.author {
                s.into()
            } else { RawString::null() },
            id: desc.id.into(),
            name: if let Some(s) = desc.name {
                s.into()
            } else { RawString::null() },
            version: desc.version.into(),
        }
    }
}

impl RustPluginDescription {
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
}

pub trait Plugin {
    fn description() -> RustPluginDescription;

    fn on_enable(&self);

    fn on_disable(&self) {
        //
    }
}

type PlugFunc = (extern fn() -> RustPluginDescFFI, extern fn(*const ()), extern fn(*const ()), extern fn(*mut ()));

fn plugin_interface<P: Plugin>() -> PlugFunc {
    let description_fun = plugin_description::<P>;
    let on_enable_fun = unsafe { mem::transmute::<_, extern fn(*const ())>(P::on_enable as fn(_)) };
    let on_disable_fun = unsafe { mem::transmute::<_, extern fn(*const ())>(P::on_disable as fn(_)) };
    let plugin_unload_fun = plugin_unload::<P>;

    (description_fun,
     on_enable_fun,
     on_disable_fun,
     plugin_unload_fun)
}

extern fn plugin_unload<P: Plugin>(instance: *mut ()) {
    unsafe { Box::<P>::from_raw(instance as _) };
}

extern fn plugin_description<P: Plugin>() -> RustPluginDescFFI {
    let desc = P::description();
    desc.into()
}