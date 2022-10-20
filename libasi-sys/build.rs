extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let platform = match env::consts::FAMILY {
        "unix" => "linux",
        "windows" => "windows",
        _ => panic!("Unsupported operating system"),
    };

    let arch = match env::consts::ARCH {
        "x86" => "x86",
        "x86_64" => "x64",
        "arm" => "armv7",
        "aarch64" => "armv8",
        _ => panic!("Unsupported CPU architecture"),
    };

    let library_path =
        std::fs::canonicalize(format!("./vendors/zwo-sdk/{}/lib/{}", platform, arch)).unwrap();
    let header_path = format!("./vendors/zwo-sdk/{}/include/ASICamera2.h", platform);

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", library_path.display());

    // Tell cargo to tell rustc to link the library.
    println!("cargo:rustc-link-lib=ASICamera2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", header_path);

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
