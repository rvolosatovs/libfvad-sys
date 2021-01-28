use std::{env, path::PathBuf};

use autotools;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        autotools::Config::new("src/libfvad")
            .build()
            .join("lib")
            .display()
    );
    println!("cargo:rustc-link-lib=static=fvad");
    println!("cargo:rustc-link-lib=c");

    bindgen::Builder::default()
        .header("src/libfvad/include/fvad.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Failed to write bindings");
}
