use std::mem;
use miraust_api::bot::Bot;
use miraust_api::event::Event;
use miraust_api::event::listener::Listener;
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
            version: "0.0.1".to_string(),
        }
    }

    fn on_enable() {
        println!("enabled!");
        for _ in 0..2 {
            let b = Bot::find_instance(1312);
            println!("{}", b.is_none());
        }


        let listener = Listener::new(|event| {
            match event {
                Event::GroupMessageEvent(g) => {

                    println!("In Rust: {}", g.message().to_string());
                }
                _ => {
                    println!("Any");
                    println!("Bot: {}", Bot::find_instance(779119352).is_some());
                }
            }
        });

        mem::forget(listener);
        //println!("{:?}", std::env::current_dir().unwrap());
    }
}