crate::ix!();
use std::io::{self, Write}; // Import io::Write for flush
use std::ptr; // Import the ptr module
fn example_function() {

    let env:                *mut jni::sys::JNIEnv          = todo!();

    // get this from the java side. it probably comes from your midi device selector
    let midi_device_obj:    jni::sys::jobject              = todo!();

    // this will be initialize by the call below
    let out_device_ptr_ptr: *mut *mut ndk_sys::AMidiDevice = todo!();

    // after this call, the initialized ndk AMidiDevice output device will be written through out_device_ptr_ptr
    //
    // you pass the JNIEnv into this call, as well as a jobject which represents the midiDevice you
    // get from java
    let status:             ndk_sys::media_status_t        = unsafe { 
        ndk_sys::AMidiDevice_fromJava(
            env,
            midi_device_obj,
            out_device_ptr_ptr
        )
    };

    println!("status={:?}",status);
}

fn load_amidi() -> Result<(), Box<dyn std::error::Error>> {
    
    unsafe {
            // Attempt to load by absolute path first for diagnostics
       // let lib_path = "/system/lib/libamidi.so"; // Correct for 32-bit target
        //println!("[RUST_TEST_DEBUG] Attempting to load by absolute path: {}", lib_path);
        //io::stdout().flush().unwrap_or_default();

        let lib = match Library::new("libamidi.so"){
        //let lib = match Library::new(lib_path) {
            Ok(l) => {
                println!("[RUST_TEST_DEBUG] Successfully loaded");
               // println!("[RUST_TEST_DEBUG] Successfully loaded from absolute path: {}", lib_path);
                //io::stdout().flush().unwrap_or_default();
                l
            }
            Err(e) => {
                println!("[RUST_TEST_DEBUG] Failed to load Trying 'libamidi.so' by name");
                //io::stdout().flush().unwrap_or_default();
                Library::new("libamidi.so")?
            }
        };

        // Try to get AMidiDevice_fromJava as an additional test
        match lib.get::<unsafe extern "C" fn(*mut jni::sys::JNIEnv, jni::sys::jobject, *mut *mut ndk_sys::AMidiDevice) -> ndk_sys::media_status_t>(b"AMidiDevice_fromJava\0") {
           Ok(_) => {
                println!("[RUST_TEST_DEBUG] Successfully got symbol 'AMidiDevice_fromJava'");
                //io::stdout().flush().unwrap_or_default();
            }
            Err(e) => {
                println!("[RUST_TEST_DEBUG] Failed to get symbol 'AMidiDevice_fromJava': {:?}", e);
                return Err(Box::new(e)); // Propagate this error to make the test fail
            }
        }

       /*  // Try to get AMidi_getVersion
        match lib.get::<unsafe extern "C" fn() -> u32>(b"AMidi_getVersion\0") {
            Ok(amidi_get_version_fn) => {
                let version = amidi_get_version_fn();
                println!("[RUST_TEST_DEBUG] AMidi_getVersion() returned: {}", version);
            }
            Err(e) => {
                println!("[RUST_TEST_DEBUG] Failed to get symbol 'AMidi_getVersion': {:?}", e);
                // Re-throw the error to make the test fail as before if this is the problematic symbol
                io::stdout().flush().unwrap_or_default();
                return Err(Box::new(e)); 
            }
        } */
 
   
    }
    Ok(())
}



#[cfg(test)]
mod test_libloading {
    use super::*;

   // #[traced_test] 
   #[test]
   fn test_load_amidi() {
        println!("[RUST_TEST_DEBUG] LD_LIBRARY_PATH: {:?}", std::env::var("LD_LIBRARY_PATH"));
       // io::stdout().flush().unwrap_or_default();
        println!("[RUST_TEST_DEBUG] RUST_BACKTRACE: {:?}", std::env::var("RUST_BACKTRACE"));
       // io::stdout().flush().unwrap_or_default();
        load_amidi().expect("expected to load amidi object");
       println!("test TEST TEST last LINe");
    }
}

#[cfg(test)]
mod test_linking {
    use super::*; // To get ndk_sys, ptr, etc.

    #[test]
    fn test_amidi_device_from_java_linkage() {
        println!("[RUST_TEST_DEBUG] Testing AMidiDevice_fromJava linkage (expects libamidi.so to be auto-linked by NDK).");

        // These are dummy values solely for testing the linkage.
        // A real, functional call would require valid JNIEnv and jobject instances
        // obtained from the Java side of an Android application.
        let dummy_env: *mut jni::sys::JNIEnv = ptr::null_mut();
        let dummy_midi_device_obj: jni::sys::jobject = ptr::null_mut(); 
        let mut out_device_ptr: *mut ndk_sys::AMidiDevice = ptr::null_mut();

        // The primary goal here is to see if this line compiles and links,
        // and if libamidi.so is loaded by the OS (check LD_DEBUG=libs output from Makefile).
        // The function call itself will likely fail (return an error status) or could
        // potentially crash if the NDK function doesn't handle null pointers gracefully,
        // but that's acceptable for a pure linkage test.
        println!("[RUST_TEST_DEBUG] Calling ndk_sys::AMidiDevice_fromJava with dummy pointers...");
        let status = unsafe {
            ndk_sys::AMidiDevice_fromJava(
                dummy_env,
                dummy_midi_device_obj,
                &mut out_device_ptr, // Pass a pointer to our out_device_ptr
            )
        };

        println!("[RUST_TEST_DEBUG] ndk_sys::AMidiDevice_fromJava call completed with status: {:?}", status);
        println!("[RUST_TEST_DEBUG] Value of out_device_ptr after call: {:?}", out_device_ptr);

        // If we reached here without a linker error during build or an OS error about libamidi.so not found at runtime, the linkage is working.
        // The LD_DEBUG=libs output (captured in test_output.txt by your Makefile) will be key to confirm libamidi.so was loaded.
    }
}
