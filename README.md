# Miraust

Mirai Rust Interface

# 自主构建

clone 本项目，在项目根目录执行`cargo b --package miraust_core`进行构建

# 插件开发示例

```rust
#[no_mangle]
extern fn on_load() -> RustPluginInterface { // 插件被加载时调用进行初始化，在插件被卸载前不会再次调用
    MyPlugin {
        listener: RefCell::new(None)
    }.into()
}

// 插件实例
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
```

# 插件的生命周期

core通过`on_load`函数获取插件实例和函数表，然后调用`Plugin::on_enable`启用插件

插件在卸载前会先`drop`插件实例，然后释放库资源