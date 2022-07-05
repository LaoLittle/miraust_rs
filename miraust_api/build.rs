fn main() {
    // $ORIGIN: origin path of bin
    println!("cargo:rustc-link-arg=-L./target/release");
    println!("cargo:rustc-link-arg=-L./target/debug");
    //println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/core");
}