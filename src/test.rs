use std::thread;

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