#![allow(unused_variables)]

use std::{env, fs, path::PathBuf};
use tracing::*;

fn main() {
    tracing_setup::configure_tracing();

    info!("Starting build.rs execution");

    // HOST is the triple for your build machine (e.g., x86_64-apple-darwin or x86_64-unknown-linux-gnu).
    // TARGET is the triple for the cross-compiled output (e.g., armv7-linux-androideabi, etc.).
    let host = env::var("HOST").expect("Missing HOST environment variable");
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("Missing CARGO_CFG_TARGET_OS environment variable");

    info!(?host, ?target_os, "Detected build environment");

    if target_os == "android" {
        if let Err(e) = android(&host) {
            error!(error = ?e, "Failed android-specific build step");
            std::process::exit(1);
        }
    } else {
        info!("No special steps required for target OS: {}", target_os);
    }

    info!("Finished build.rs execution");
}

fn android(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Configuring build for Android");

    // Emit the linker instruction for c++_shared
    println!("cargo:rustc-link-lib=c++_shared");
    debug!("Emitted linker instruction for c++_shared");

    // We'll retrieve both the final output path and the target triple from env vars
    let output_path = env::var("CARGO_NDK_OUTPUT_PATH")
        .unwrap_or_else(|_| "./target/ndk-output".into());
    debug!(output_path, "Retrieved or defaulted CARGO_NDK_OUTPUT_PATH");

    let target_triple = env::var("TARGET")
        .expect("Missing TARGET environment variable");
    debug!(?target_triple, "Detected TARGET triple from environment");

    // Some NDK folder names differ from Rust’s target triple (e.g. Rust uses armv7-linux-androideabi,
    // but the folder in the NDK is arm-linux-androideabi). We fix up certain known cases below.
    let ndk_lib_subdir = fixup_target_triple(&target_triple);
    debug!(?ndk_lib_subdir, "Resolved subdir name in the NDK for libc++_shared");

    // We'll pick either `darwin-x86_64` or `linux-x86_64` for the path to libc++_shared
    // under $NDK_HOME/toolchains/llvm/prebuilt/. If you're on Windows, adjust as needed.
    let host_prebuilt = if host.contains("apple-darwin") {
        "darwin-x86_64"
    } else if host.contains("linux") {
        "linux-x86_64"
    } else {
        return Err(format!("Unsupported build host for prebuilt toolchain: {}", host).into());
    };
    debug!(host_prebuilt, "Inferred prebuilt subdirectory name based on HOST");

    let ndk_home = env::var("ANDROID_NDK_HOME")
        .expect("Missing ANDROID_NDK_HOME");
    debug!(ndk_home, "Retrieved ANDROID_NDK_HOME");

    // Construct the full path to libc++_shared.so
    let libcxx_shared_path = PathBuf::from(&ndk_home)
        .join("toolchains/llvm/prebuilt")
        .join(host_prebuilt)
        .join("sysroot")
        .join("usr")
        .join("lib")
        .join(ndk_lib_subdir)
        .join("libc++_shared.so");

    debug!(?libcxx_shared_path, "Constructed libc++_shared.so source path");

    if !libcxx_shared_path.exists() {
        error!(?libcxx_shared_path, "libc++_shared.so not found");
        return Err(format!("Could not find libc++_shared.so at {:?}", libcxx_shared_path).into());
    }

    let target_output_path = PathBuf::from(&output_path)
        .join(&target_triple)
        .join("libc++_shared.so");

    debug!(?target_output_path, "Constructed libc++_shared.so target path");

    fs::create_dir_all(target_output_path.parent().unwrap())?;
    debug!(parent = ?target_output_path.parent(), "Ensured parent directory exists");

    fs::copy(&libcxx_shared_path, &target_output_path)?;
    info!(
        from = ?libcxx_shared_path,
        to = ?target_output_path,
        "Copied libc++_shared.so successfully"
    );

    Ok(())
}

/// For certain Rust target triples (e.g., armv7-linux-androideabi),
/// the corresponding folder in the NDK is named differently.
/// This function maps known Rust triples to the appropriate subdir.
fn fixup_target_triple(triple: &str) -> &str {
    match triple {
        // Rust says armv7, NDK folder says arm
        "armv7-linux-androideabi" | "thumbv7neon-linux-androideabi" => "arm-linux-androideabi",
        // 64-bit ARM is typically aarch64-linux-android
        // (this one usually matches what's on disk)
        _ => triple,
    }
}
