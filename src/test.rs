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