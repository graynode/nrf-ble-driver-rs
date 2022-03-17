extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=nrf-ble-driver-4.1.4-linux_x86_64/lib");
    println!("cargo:rustc-link-lib=nrf-ble-driver-sd_api_v5");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Inrf-ble-driver-4.1.4-linux_x86_64/include/sd_api_v5")
        .header("wrapper.h")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}