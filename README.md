# Miraust
Mirai Rust Interface

# 自主构建
clone 本项目，在项目根目录执行`cargo b --package miraust_core`进行构建

# 插件开发示例
```rust
#[no_mangle] // 插件被加载时调用进行初始化，在插件被卸载前不会再次调用
extern fn on_load() -> RustPluginFunc {
    A.register()
}

// 插件实例
struct A;

impl ToMirai for A {
    // 插件描述
    fn description() -> RustPluginDescription {
        RustPluginDescription {
            author: None, // 插件作者
            id: "org.laolittle".to_string(), // 插件id
            name: None, // 插件名称
            version: "0.0.1".to_string(), // 插件版本
        }
    }

    // 插件启用阶段被调用，可能调用多次
    fn on_enable() {
        let listener = Listener::new(|event| {
            match event {
                Event::GroupMessageEvent(g) => {
                    let m = g.message();

                    if m.content() == "测试" {
                        let chain = MessageChain::builder()
                            .add("嘻嘻")
                            .add("\nWow")
                            .build();

                        g.subject().send_message(&chain);
                    }
                }

                Event::FriendMessageEvent(_) => {}
                _ => {}
            }
        });

        // listener 被`drop`会自动停止监听，
        // 也可以通过`listener.complete()`主动停止监听
        mem::forget(listener);
    }
}
```