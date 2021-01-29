extern crate cc;
use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .include("src/libfvad/src")
        .include("src/libfvad/src/vad")
        .include("src/libfvad/src/signal_processing")
        .file("src/libfvad/src/signal_processing/division_operations.c")
        .file("src/libfvad/src/signal_processing/get_scaling_square.c")
        .file("src/libfvad/src/signal_processing/resample_48khz.c")
        .file("src/libfvad/src/signal_processing/resample_by_2_internal.c")
        .file("src/libfvad/src/signal_processing/resample_fractional.c")
        .file("src/libfvad/src/signal_processing/spl_inl.c")
        .file("src/libfvad/src/signal_processing/energy.c")
        .file("src/libfvad/src/vad/vad_core.c")
        .file("src/libfvad/src/vad/vad_filterbank.c")
        .file("src/libfvad/src/vad/vad_gmm.c")
        .file("src/libfvad/src/vad/vad_sp.c")
        .file("src/libfvad/src/fvad.c")
        .flag_if_supported("-O2")
        .compile("libfvad.a");

    bindgen::Builder::default()
        .header("src/libfvad/include/fvad.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Failed to write bindings");
}
