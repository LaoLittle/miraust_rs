pub struct RustPluginFunc {
    description_fun: fn() -> RustPluginDescription,
    on_enable_fun: fn(),
    on_disable_fun: fn(),
}

#[derive(Debug)]
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

pub trait ToMirai {
    fn description() -> RustPluginDescription;

    fn on_enable();

    fn on_disable() {
        //
    }

    fn register(&self) -> RustPluginFunc {
        RustPluginFunc { description_fun: Self::description, on_enable_fun: Self::on_enable, on_disable_fun: Self::on_disable }
    }
}