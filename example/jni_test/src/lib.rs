use miraust::bot::bot::Bot;
use miraust::plugin::{RustPluginDescription, RustPluginFunc, ToMirai};

#[no_mangle]
fn on_load() -> RustPluginFunc {

    A.register()
}


struct A;

impl ToMirai for A{
    fn description() -> RustPluginDescription {
        RustPluginDescription {
            author: Some(String::from("laolittle")),
            id: "org.laolittle".to_string(),
            name: Some(String::from("test")),
            version: "0.1.0".to_string()
        }
    }

    fn on_enable() {
        //let bot = Bot::get_instance_uncheck(1323);
        println!("enabled!");
    }
}