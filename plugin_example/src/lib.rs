use miraust_api::bot::Bot;
use miraust_api::plugin::{RustPluginDescription, RustPluginFunc, ToMirai};

#[no_mangle]
fn on_load() -> RustPluginFunc {
    A.register()
}

struct A;

impl ToMirai for A {
    fn description() -> RustPluginDescription {
        RustPluginDescription {
            author: None,
            id: "org.laolittle".to_string(),
            name: None,
            version: "0.0.1".to_string()
        }
    }

    fn on_enable() {
        println!("enabled!");
        let b = Bot::find_instance(1312);
        println!("{}", b.is_none());
        //println!("{:?}", std::env::current_dir().unwrap());
    }
}