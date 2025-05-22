#![allow(unused_variables)]

use std::{env, fs, path::PathBuf};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    let output_path = env::var("CARGO_NDK_OUTPUT_PATH").unwrap();

    //let target_triple = env::var("CARGO_NDK_ANDROID_TARGET").unwrap();
    let target_triple = "arm-linux-androideabi";

    let ndk_home = env::var("ANDROID_NDK_HOME").unwrap();

    //let api_level = env::var("CARGO_NDK_ANDROID_PLATFORM_LEVEL").unwrap();
    let api_level = "r27c";

    let libcxx_shared_path = PathBuf::from(ndk_home)
        .join("toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib")
        .join(&target_triple)
        //.join(&api_level)
        .join("libc++_shared.so");

    assert!(
        libcxx_shared_path.exists(),
        "Could not find libc++_shared.so at {:?}",
        libcxx_shared_path
    );

    let target_output_path = PathBuf::from(output_path)
        .join(&target_triple)
        .join("libc++_shared.so");

    fs::create_dir_all(target_output_path.parent().unwrap()).unwrap();

    fs::copy(libcxx_shared_path, target_output_path).unwrap();
}

