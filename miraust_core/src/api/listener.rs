use tokio::task::JoinHandle;

use crate::event::EventManaged;
use crate::jni_ffi::jni_callback::{CALLBACK_POOL, MIRAI_ENV};
use crate::managed::Managed;

type Listener = JoinHandle<()>;

#[no_mangle]
extern fn listener_subscribe_always(f: Box<dyn Fn(*mut (), u8) + Send + 'static>) -> *mut Listener {
    let runtime = CALLBACK_POOL.get().unwrap();
    let mut r = MIRAI_ENV.get().unwrap().sender.subscribe();

    let handle = runtime.spawn(async move {
        while let Ok(event) = r.recv().await {
            f(Box::into_raw(Box::new(event.0)) as *mut (), event.1);
        }
    });
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
extern fn listener_stop(handle: &Listener) {
    handle.abort();
}