extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default().header("wrapper.h");
    if let Ok(r) = env::var("SYSROOT") {
        builder = builder.clang_arg(format!("--sysroot={}", r))
    };
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

