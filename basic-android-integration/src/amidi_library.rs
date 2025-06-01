// ---------------- [ File: basic-android-integration/src/amidi_library.rs ]
crate::ix!();

#[derive(Debug, Getters, Setters)]
#[getset(get = "pub")]
pub struct AmidiLibrary {
    /// Keep the dynamic library alive. If this is dropped, all function pointers
    /// become invalid. So we store it in an Arc.
    library: Arc<Library>,

    /// Now each function pointer is a raw pointer function type, not a Symbol.
    /// This avoids self-referential lifetime issues.
    pub(crate) amidi_device_from_java:               AMidiDeviceFromJavaFn,
    pub(crate) amidi_device_release:                 AMidiDeviceReleaseFn,
    pub(crate) amidi_device_get_type:                AMidiDeviceGetTypeFn,
    pub(crate) amidi_device_get_num_input_ports:     AMidiDeviceGetNumInputPortsFn,
    pub(crate) amidi_device_get_num_output_ports:    AMidiDeviceGetNumOutputPortsFn,
    pub(crate) amidi_device_get_default_protocol:    AMidiDeviceGetDefaultProtocolFn,

    pub(crate) amidi_output_port_open:               AMidiOutputPortOpenFn,
    pub(crate) amidi_output_port_close:              AMidiOutputPortCloseFn,
    pub(crate) amidi_output_port_receive:            AMidiOutputPortReceiveFn,

    pub(crate) amidi_input_port_open:                AMidiInputPortOpenFn,
    pub(crate) amidi_input_port_send:                AMidiInputPortSendFn,
    pub(crate) amidi_input_port_send_with_timestamp: AMidiInputPortSendWithTimestampFn,
    pub(crate) amidi_input_port_send_flush:          AMidiInputPortSendFlushFn,
    pub(crate) amidi_input_port_close:               AMidiInputPortCloseFn,
}

impl AmidiLibrary {
    /// Load the library at `path` and resolve all symbols.
    /// Each symbol is read out to a plain function pointer.
    /// We keep an Arc<Library> so the library remains loaded as long as
    /// AmidiLibrary is alive.
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Attempting to load Amidi library from: {}", path);

        // SAFETY: Loading a shared library is inherently unsafe, but typical usage.
        // We do so in a safe function, which is acceptable if we trust the library path.
        let lib = unsafe { Library::new(path)? };
        let lib_arc = Arc::new(lib);

        // For each symbol, we do:
        //   let sym = unsafe { lib_arc.get::<SomeFn>(b"symbol_name")? };
        //   let function_pointer = *sym;
        // This yields a plain function pointer that we store in self.

        let amidi_device_from_java: AMidiDeviceFromJavaFn = unsafe {
            *lib_arc.get(b"AMidiDevice_fromJava")?
        };
        let amidi_device_release: AMidiDeviceReleaseFn = unsafe {
            *lib_arc.get(b"AMidiDevice_release")?
        };
        let amidi_device_get_type: AMidiDeviceGetTypeFn = unsafe {
            *lib_arc.get(b"AMidiDevice_getType")?
        };
        let amidi_device_get_num_input_ports: AMidiDeviceGetNumInputPortsFn = unsafe {
            *lib_arc.get(b"AMidiDevice_getNumInputPorts")?
        };
        let amidi_device_get_num_output_ports: AMidiDeviceGetNumOutputPortsFn = unsafe {
            *lib_arc.get(b"AMidiDevice_getNumOutputPorts")?
        };
        let amidi_device_get_default_protocol: AMidiDeviceGetDefaultProtocolFn = unsafe {
            *lib_arc.get(b"AMidiDevice_getDefaultProtocol")?
        };

        let amidi_output_port_open: AMidiOutputPortOpenFn = unsafe {
            *lib_arc.get(b"AMidiOutputPort_open")?
        };
        let amidi_output_port_close: AMidiOutputPortCloseFn = unsafe {
            *lib_arc.get(b"AMidiOutputPort_close")?
        };
        let amidi_output_port_receive: AMidiOutputPortReceiveFn = unsafe {
            *lib_arc.get(b"AMidiOutputPort_receive")?
        };

        let amidi_input_port_open: AMidiInputPortOpenFn = unsafe {
            *lib_arc.get(b"AMidiInputPort_open")?
        };
        let amidi_input_port_send: AMidiInputPortSendFn = unsafe {
            *lib_arc.get(b"AMidiInputPort_send")?
        };
        let amidi_input_port_send_with_timestamp: AMidiInputPortSendWithTimestampFn = unsafe {
            *lib_arc.get(b"AMidiInputPort_sendWithTimestamp")?
        };
        let amidi_input_port_send_flush: AMidiInputPortSendFlushFn = unsafe {
            *lib_arc.get(b"AMidiInputPort_sendFlush")?
        };
        let amidi_input_port_close: AMidiInputPortCloseFn = unsafe {
            *lib_arc.get(b"AMidiInputPort_close")?
        };

        info!("Successfully loaded libamidi.so and all symbols.");

        Ok(Self {
            library: lib_arc,

            amidi_device_from_java,
            amidi_device_release,
            amidi_device_get_type,
            amidi_device_get_num_input_ports,
            amidi_device_get_num_output_ports,
            amidi_device_get_default_protocol,

            amidi_output_port_open,
            amidi_output_port_close,
            amidi_output_port_receive,

            amidi_input_port_open,
            amidi_input_port_send,
            amidi_input_port_send_with_timestamp,
            amidi_input_port_send_flush,
            amidi_input_port_close,
        })
    }

    /// Optional helper to return an Arc<Self> in one go.
    pub fn load_arc(path: &str) -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        Ok(Arc::new(Self::new(path)?))
    }
}

#[cfg(test)]
mod test_amidi_library {
    use super::*;
    use tracing::info;

    #[test]
    fn test_library_load_failure() {
        info!("Testing error on invalid path...");
        // Attempt to load a nonsense path. We expect an error.
        let result = AmidiLibrary::new("does_not_exist_libamidi.so");
        assert!(result.is_err(), "Should fail to load invalid library path");
    }

    // If you have a real path and environment, you can test success like:
    // #[test]
    // fn test_library_load_success() {
    //     let lib = AmidiLibrary::new("/path/to/libamidi.so")
    //         .expect("Should load libamidi successfully");
    //     info!("Successfully loaded the library in test: {:?}", lib);
    // }
}

