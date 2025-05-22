crate::ix!();

fn example_function() {

    let env:                *mut jni::sys::JNIEnv          = todo!();
    let midi_device_obj:    jni::sys::jobject              = todo!();
    let out_device_ptr_ptr: *mut *mut ndk_sys::AMidiDevice = todo!();

    let status:             ndk_sys::media_status_t        = unsafe { 
        ndk_sys::AMidiDevice_fromJava(
            env,
            midi_device_obj,
            out_device_ptr_ptr
        )
    };

    println!("status={:?}",status);
}
