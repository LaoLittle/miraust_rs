use std::cell::RefCell;

use miraust_api::bot::Bot;
use miraust_api::event::Event;
use miraust_api::event::listener::Listener;
use miraust_api::message::chain::MessageChain;
use miraust_api::plugin::{Plugin, RustPluginDescription, RustPluginInterface};

#[no_mangle]
extern fn on_load() -> RustPluginInterface {
    MyPlugin {
        listener: RefCell::new(None)
    }.into()
}

struct MyPlugin {
    listener: RefCell<Option<Listener>>,
}

impl Plugin for MyPlugin {
    // 插件描述
    fn description() -> RustPluginDescription {
        RustPluginDescription {
            author: None,
            id: "org.laolittle".to_string(),
            name: None,
            version: "0.0.1".to_string(),
        }
    }

    // 插件启用阶段被调用，可能调用多次（插件重复启动禁用），但并不会重置插件实例
    fn on_enable(&self) {
        println!("enabled!");
        for _ in 0..2 {
            let b = Bot::find_instance(1312);
            println!("{}", b.is_none());
        }

        // listener 被`drop`会自动停止监听，
        // 也可以通过`listener.complete()`主动停止监听
        let listener = Listener::new(|event| {
            match event {
                Event::GroupMessageEvent(g) => {
                    let m = g.message();

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

        let mut r = self.listener.borrow_mut();
        *r = Some(listener);
    }
}

// 插件卸载前会先`drop`插件实例
impl Drop for MyPlugin {
    fn drop(&mut self) {
        println!("Dropping");
    }
}