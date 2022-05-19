use tokio::task::JoinHandle;
use crate::event::Event;
use crate::jni_ffi::jni_callback::{CALLBACK_POOL, MIRAI_ENV};

#[no_mangle]
extern "Rust" fn event_spawn_subscribe_always(f: Box<dyn Fn(Event) + Send + 'static>) -> JoinHandle<()> {
    let runtime = CALLBACK_POOL.get().unwrap();
    let mut r = MIRAI_ENV.get().unwrap().sender.subscribe();

    runtime.spawn(async move {
        loop {
            let event = r.recv().await.expect("?");
            f(event);
        }
    })
}

#[no_mangle]
extern "Rust" fn listener_stop(handle: JoinHandle<()>) {
    handle.abort();
}