#[cfg(test)]
mod tests {
    use std::io::repeat;
    use std::sync::{Arc, mpsc};
    use std::thread;
    use std::thread::JoinHandle;
    use tokio::runtime::Runtime;
    use tokio::task::yield_now;

    #[test]
    fn lib_load() {
        use crate::plugin::RustPluginFunc;
        let lib = unsafe { libloading::Library::new("target/debug/libjni_test.so").unwrap() };

        let f = unsafe {
            lib.get::<fn() -> RustPluginFunc>(b"on_load").unwrap()
        };

        let _func: RustPluginFunc = f();
    }

    #[test]
    fn vec() {
        let mut a = vec![Box::new(1)];

        a.push(Box::new(2));
        a.push(Box::new(33));

        a.remove(0);

        println!("{:?}", a.get(0));
    }

    #[test]
    fn ju() {
        use jni::signature::{JavaType, TypeSignature};
        let t = TypeSignature::from_str("(V)Ld;").unwrap();

        match t.ret {
            JavaType::Object(ref s) => println!("{}", s),
            _ => println!("?"),
        }

        println!("{}", t.ret);
    }

    #[test]
    fn p() {
        unsafe {
            let a = Box::<u16>::new(0b1000000011000000);
            let c = &*a as *const u16 as *mut u8;

            *c = 128;

            println!("{}", a);
        }
    }

    /*#[test]
    fn channel() {
        let (sender, recv) = crossbeam::channel::unbounded::<i32>();
        drop(sender);
        thread::spawn(move || {
            let recv = recv;
            println!("Start");
            while let Ok(a) = recv.recv() {
                println!("{a}");
            }
            println!("Exit!");
        });
    }*/

    #[test]
    fn shit() {
        let (s, r) = mpsc::channel::<JoinHandle<()>>();
        let handle = thread::spawn(move || {
            let handle = r.recv().unwrap();
            handle.join();
        });

        s.send(handle);
    }

    #[test]
    fn broadcast_test() {
        let (s, mut rx1) = tokio::sync::broadcast::channel::<&str>(13);

        let mut rx2 = s.subscribe();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .worker_threads(4)
            .on_thread_start(|| {
                println!("{:?} start!", thread::current());
            })
            .build().unwrap();

        runtime.spawn(async move {
            let a = rx1.recv().await.unwrap();
            yield_now().await;
        });

        let handle = runtime.spawn(async move {
            while let Ok(b) = rx2.recv().await {
                println!("{}", b);
            }
        });

        for _ in 0..=5 {
            s.send("嘻嘻鲨了你").unwrap();
        }
        println!("??");
        runtime.block_on(async {
            //handle.await.unwrap();
        });

        //thread::sleep(Duration::from_millis(12));
    }

    #[test]
    fn runtime() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(16)
            .on_thread_start(|| {
                println!("?");
            })
            .build().unwrap();

        let rt = Arc::new(rt);

        let rt0 = rt.clone();
        thread::spawn(move || {
            loop {
                rt0.spawn(async {
                    println!("Hello0!");

                });
            }
        });

        let rt1 = rt.clone();
        let t2 = thread::spawn(move || {
            println!("{:?}", thread::current());
            rt1.spawn(async {
                println!("{:?}", thread::current());
                println!("Hello1!");
            });
        });

        rt.block_on(async {

            println!("12");
        });

        t2.join().unwrap();
    }

    #[test]
    fn rt_drop() {
        fn start(runtime: &Runtime) {
            drop(runtime.spawn(async {
                println!("!@#@#%&(*&!#(*&");
            }))
        }

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(16)
            .build().unwrap();

        let rt = Arc::new(rt);
        let rt0 = rt.clone();
        let handle = thread::spawn(move || {
            start(&*rt0);
            start(&*rt0);
        });

        start(&rt);
        handle.join().unwrap();
    }
}