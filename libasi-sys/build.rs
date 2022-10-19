extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let path = std::fs::canonicalize("./vendors/zwo-sdk/linux/lib/x64");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", path.unwrap().display());

    // Tell cargo to tell rustc to link the library.
    println!("cargo:rustc-link-lib=ASICamera2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=./vendors/zwo-sdk/linux/include/ASICamera2.h");

    let bindings = bindgen::Builder::default()
        .header("./vendors/zwo-sdk/linux/include/ASICamera2.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
