#![allow(unused_variables)]

use std::{env, fs, path::PathBuf};
use tracing::*;

fn main() {

    tracing_setup::configure_tracing();

    info!("Starting build.rs execution");

    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("Missing CARGO_CFG_TARGET_OS environment variable");

    info!(target_os, "Detected build target OS");

    if target_os == "android" {
        if let Err(e) = android() {
            error!(error = ?e, "Failed android-specific build step");
            std::process::exit(1);
        }
    } else {
        info!("No special steps required for target OS: {}", target_os);
    }

    info!("Finished build.rs execution");
}

fn android() -> Result<(), Box<dyn std::error::Error>> {
    info!("Configuring build for Android");

    println!("cargo:rustc-link-lib=c++_shared");
    debug!("Configured cargo to link library 'c++_shared'");
        
     println!("cargo:rustc-link-lib=amidi");
println!("cargo:rustc-link-search=/home/loko/Android/Sdk/ndk/27/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/30/");

    let output_path = env::var("CARGO_NDK_OUTPUT_PATH")
        .unwrap_or_else(|_| "./target/ndk-output".into());
    debug!(output_path, "Retrieved or defaulted CARGO_NDK_OUTPUT_PATH");
//let target_triple= "aarch64-linux-android";
    let target_triple = "arm-linux-androideabi";
    warn!(target_triple, "Using hardcoded target triple");

    let ndk_home = env::var("ANDROID_NDK_HOME")
        .expect("Missing ANDROID_NDK_HOME");
    debug!(ndk_home, "Retrieved ANDROID_NDK_HOME");

    let libcxx_shared_path = PathBuf::from(&ndk_home)
        .join("toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib")
        .join(&target_triple)
        .join("libc++_shared.so");

    debug!(?libcxx_shared_path, "Constructed libc++_shared.so source path");

    if !libcxx_shared_path.exists() {
        error!(?libcxx_shared_path, "libc++_shared.so not found");
        return Err(format!("Could not find libc++_shared.so at {:?}", libcxx_shared_path).into());
    }

    let mut target_output_path = PathBuf::from(output_path.clone())
        .join(&target_triple)
        .join("libc++_shared.so");

    debug!(?target_output_path, "Constructed libc++_shared.so target path");

    fs::create_dir_all(target_output_path.parent().unwrap())?;
    debug!(parent = ?target_output_path.parent(), "Ensured parent directory exists");

    fs::copy(&libcxx_shared_path, &target_output_path)?;
    info!(from = ?libcxx_shared_path, to = ?target_output_path, "Copied libc++_shared.so successfully");

//------
 let libamidi_path = PathBuf::from(&ndk_home)
        .join("toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib")
        .join("arm-linux-androideabi")
        .join("30")
        .join("libamidi.so");

    debug!(?libamidi_path, "Constructed libamidi.so source path");

    if !libamidi_path.exists() {
        error!(?libamidi_path, "libamidi.so not found");
        return Err(format!("Could not find libamidi.so at {:?}", libamidi_path).into());
    }

     target_output_path = PathBuf::from(output_path)
        .join(&target_triple)
        .join("libamidi.so");

    debug!(?target_output_path, "Constructed libamidi.so target path");

    fs::create_dir_all(target_output_path.parent().unwrap())?;
    debug!(parent = ?target_output_path.parent(), "Ensured parent directory exists");

    fs::copy(&libamidi_path, &target_output_path)?;
    info!(from = ?libamidi_path, to = ?target_output_path, "Copied libamidi.so successfully");
//_--


    Ok(())
}
