#[cfg(test)]
mod tests {
    use std::lazy::OnceCell;
    use std::sync::mpsc;
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use jni::JavaVM;

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

    #[test]
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
    }

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
        let (s,mut rx1) = tokio::sync::broadcast::channel::<&str>(13);

        let mut rx2 = s.subscribe();

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(3)
            .on_thread_start(|| {
                println!("{:?} start!", thread::current());
            })
            .build().unwrap();

        runtime.spawn(async move {
            let a = rx1.recv().await.unwrap();

            println!("{}", a);
        });

        let handle = runtime.spawn(async move {
            let b = rx2.recv().await.unwrap();

            println!("{}", b);
        });

        s.send("xi xi, sha le ni").unwrap();

        //thread::sleep(Duration::from_millis(12));
    }
}