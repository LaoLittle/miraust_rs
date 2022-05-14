use crate::plugin::RustPluginFunc;

#[test]
pub fn lib_load() {
    let lib = unsafe { libloading::Library::new("target/debug/libjni_test.so").unwrap() };

    let f = unsafe {
        lib.get::<fn() -> RustPluginFunc>(b"on_load").unwrap()
    };

    let func: RustPluginFunc = f();
}

#[test]
fn vec() {
    let mut a = vec![Box::new(1)];

    a.push(Box::new(2));
    a.push(Box::new(33));

    a.remove(0);

    println!("{:?}", a.get(0));
}