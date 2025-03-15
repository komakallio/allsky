extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let platform = match env::consts::FAMILY {
        "unix" => "linux",
        "windows" => "win",
        _ => panic!("Unsupported operating system"),
    };

    let arch = match env::consts::ARCH {
        "x86" => "x86",
        "x86_64" => "x64",
        "aarch64" => "arm64",
        _ => panic!("Unsupported CPU architecture"),
    };

    let library_path =
        std::fs::canonicalize(format!("./vendors/toupcamsdk/{}/{}", platform, arch)).unwrap();
    let header_path = std::fs::canonicalize("./vendors/toupcamsdk/inc/toupcam.h").unwrap();

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", library_path.display());

    // Tell cargo to tell rustc to link the library.
    println!("cargo:rustc-link-lib=toupcam");

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
