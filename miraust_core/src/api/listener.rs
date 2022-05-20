use tokio::task::JoinHandle;
use crate::event::{Event, EventManaged};
use crate::jni_ffi::jni_callback::{CALLBACK_POOL, MIRAI_ENV};
use crate::managed::Managed;

type Listener = JoinHandle<()>;
#[no_mangle]
extern fn listener_subscribe_always(f: Box<dyn Fn(EventManaged) + Send + 'static>) -> *mut Listener {
    let runtime = CALLBACK_POOL.get().unwrap();
    let mut r = MIRAI_ENV.get().unwrap().sender.subscribe();

    let handle = runtime.spawn(async move {
        loop {
            let event = r.recv().await.expect("?");
            let managed = EventManaged { inner: Managed::new(Box::into_raw(Box::new(event)) as *mut (), 12) };
            f(managed);
        }
    });
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
extern fn listener_stop(handle: &Listener) {
    handle.abort();
}