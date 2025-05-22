crate::ix!();

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
