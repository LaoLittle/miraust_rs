use std::any::Any;
use std::mem;

use miraust_api::bot::Bot;
use miraust_api::event::Event;
use miraust_api::event::listener::Listener;
use miraust_api::message::chain::MessageChain;
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
                    let m = g.message();

                    let bb: &dyn Any = &m;

                    if m.content() == "testrs" {
                        let chain = MessageChain::builder()
                            .add("嘻嘻")
                            .add("\nTesting")
                            .build();

                        println!("ok!");

                        g.subject().send_message(&chain);
                    }
                }
                Event::FriendMessageEvent(_) => {}
                _ => {}
            }
        });

        mem::forget(listener);
        //println!("{:?}", std::env::current_dir().unwrap());
    }
}