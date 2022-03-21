extern crate bindgen;


use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use flate2::read::GzDecoder;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use tar::Archive;
#[cfg(target_os = "windows")]
use zip::read::ZipArchive;

const DOWNLOAD_BASE_URL: &str = "https://github.com/NordicSemiconductor/pc-ble-driver/releases/download";
const VERSION: &str = "4.1.4";
const FILE_PREFIX: &str = "nrf-ble-driver";
#[cfg(target_os = "linux")]
const FILE_SUFFIX: &str = "linux_x86_64";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const FILE_EXT: &str = "tar.gz";
#[cfg(target_os = "windows")]
const FILE_SUFFIX: &str = "win_x86_64";
#[cfg(target_os = "windows")]
const FILE_EXT: &str = "zip";
#[cfg(target_os = "macos")]
const FILE_SUFFIX: &str = "macos_x86_64";



fn ble_driver_url() -> String {
    return format!("{}/v{}/{}-{}-{}.{}", DOWNLOAD_BASE_URL, VERSION, FILE_PREFIX, VERSION, FILE_SUFFIX, FILE_EXT);
}

fn ble_driver_file() -> String {
    return format!("{}-{}-{}.{}", FILE_PREFIX, VERSION, FILE_SUFFIX, FILE_EXT);
}

fn ble_driver_root_dir() -> String {
    return format!("{}/{}", env::var("OUT_DIR").unwrap(), ble_driver_folder());
}

fn ble_driver_folder() -> String {
    return format!("{}-{}-{}", FILE_PREFIX, VERSION, FILE_SUFFIX);
}

fn main() {
    get_nordic_ble_driver();
    extract_ble_driver();

    
    println!("cargo:rustc-link-search=native={}/lib", ble_driver_root_dir());
    println!("cargo:rustc-link-lib=nrf-ble-driver-sd_api_v5");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include/sd_api_v5", ble_driver_folder()))
        .header("wrapper.h")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_nordic_ble_driver() {
    let mut response = reqwest::blocking::get(ble_driver_url()).unwrap();
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();

    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path = out_path.join(ble_driver_file());
    let mut f = File::create(out_path).unwrap();
    f.write_all(&buf).unwrap();
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn extract_ble_driver() {
    let out_path = PathBuf::from(format!("{}/{}", env::var("OUT_DIR").unwrap(), ble_driver_file()));
    let targ_gz = File::open(out_path).unwrap();
    let tar = GzDecoder::new(targ_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(env::var("OUT_DIR").unwrap());
}

#[cfg(target_os = "windows")]
fn extract_ble_driver() {
    let zip_file = File::open(format!("{}/{}", env::var("OUT_DIR").unwrap(), ble_driver_file())).unwrap();
    let mut zip = ZipArchive::new(zip_file).unwrap();
    zip.extract(env::var("OUT_DIR").unwrap()).unwrap();
}
