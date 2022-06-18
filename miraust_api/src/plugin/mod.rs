use std::mem;

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
            plugin_unload_fun
        }
    }
}

#[repr(C)]
pub struct RustPluginInterface {
    instance: *mut (),
    description_fun: extern fn() -> RustPluginDescription,
    on_enable_fun: fn(*const ()),
    on_disable_fun: fn(*const ()),
    plugin_unload_fun: fn(*mut ()),
}

#[derive(Debug)]
#[repr(C)] // fix later: use RawString
pub struct RustPluginDescription {
    pub author: Option<String>,
    pub id: String,
    pub name: Option<String>,
    pub version: String,
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

type PlugFunc = (extern fn() -> RustPluginDescription, fn(*const ()), fn(*const ()), fn(*mut ()));

fn plugin_interface<P: Plugin>() -> PlugFunc {
    let description_fun = plugin_description::<P>;
    let on_enable_fun = unsafe { mem::transmute::<_, fn(*const ())>(P::on_enable as fn(_)) };
    let on_disable_fun = unsafe { mem::transmute::<_, fn(*const ())>(P::on_disable as fn(_)) };
    let plugin_unload_fun = plugin_unload::<P>;

    (description_fun,
     on_enable_fun,
     on_disable_fun,
     plugin_unload_fun)
}

fn plugin_unload<P: Plugin>(instance: *mut ()) {
    unsafe { Box::<P>::from_raw(instance as _) };
}

extern fn plugin_description<P: Plugin>() -> RustPluginDescription {
    P::description()
}