crate::ix!();

///////////////////////////////////////////////////////////////////////////////
// Main loader struct: AmidiLibrary
// Holds a `libloading::Library` plus all function pointers for the full API.
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Builder, Getters, Setters)]
#[builder(setter(into))]
#[getset(get = "pub")]
pub struct AmidiLibrary {
    /// The loaded libamidi.so library handle.
    #[builder(private)]
    library: Library,

    /// The symbol for `AMidiDevice_fromJava`.
    #[builder(private)]
    amidi_device_from_java: Symbol<'static, AMidiDeviceFromJavaFn>,

    /// The symbol for `AMidiDevice_release`.
    #[builder(private)]
    amidi_device_release: Symbol<'static, AMidiDeviceReleaseFn>,

    /// The symbol for `AMidiDevice_getType`.
    #[builder(private)]
    amidi_device_get_type: Symbol<'static, AMidiDeviceGetTypeFn>,

    /// The symbol for `AMidiDevice_getNumInputPorts`.
    #[builder(private)]
    amidi_device_get_num_input_ports: Symbol<'static, AMidiDeviceGetNumInputPortsFn>,

    /// The symbol for `AMidiDevice_getNumOutputPorts`.
    #[builder(private)]
    amidi_device_get_num_output_ports: Symbol<'static, AMidiDeviceGetNumOutputPortsFn>,

    /// The symbol for `AMidiDevice_getDefaultProtocol`.
    #[builder(private)]
    amidi_device_get_default_protocol: Symbol<'static, AMidiDeviceGetDefaultProtocolFn>,

    /// The symbol for `AMidiOutputPort_open`.
    #[builder(private)]
    amidi_output_port_open: Symbol<'static, AMidiOutputPortOpenFn>,

    /// The symbol for `AMidiOutputPort_close`.
    #[builder(private)]
    amidi_output_port_close: Symbol<'static, AMidiOutputPortCloseFn>,

    /// The symbol for `AMidiOutputPort_receive`.
    #[builder(private)]
    amidi_output_port_receive: Symbol<'static, AMidiOutputPortReceiveFn>,

    /// The symbol for `AMidiInputPort_open`.
    #[builder(private)]
    amidi_input_port_open: Symbol<'static, AMidiInputPortOpenFn>,

    /// The symbol for `AMidiInputPort_send`.
    #[builder(private)]
    amidi_input_port_send: Symbol<'static, AMidiInputPortSendFn>,

    /// The symbol for `AMidiInputPort_sendWithTimestamp`.
    #[builder(private)]
    amidi_input_port_send_with_timestamp: Symbol<'static, AMidiInputPortSendWithTimestampFn>,

    /// The symbol for `AMidiInputPort_sendFlush`.
    #[builder(private)]
    amidi_input_port_send_flush: Symbol<'static, AMidiInputPortSendFlushFn>,

    /// The symbol for `AMidiInputPort_close`.
    #[builder(private)]
    amidi_input_port_close: Symbol<'static, AMidiInputPortCloseFn>,
}

impl AmidiLibrary {
    /// Creates a builder for the library. Typically you only need
    /// to set the library path (`.path("libamidi.so")`).
    pub fn builder() -> AmidiLibraryBuilder {
        AmidiLibraryBuilder::default()
    }
}

impl AmidiLibraryBuilder {
    /// Required path to `libamidi.so`.
    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    /// Build and load the library, resolving all symbols.
    /// Returns an error if any symbol is missing or if the library can't be opened.
    pub fn build(&self) -> Result<Arc<AmidiLibrary>, Box<dyn std::error::Error>> {
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| "Missing path to libamidi.so in AmidiLibraryBuilder")?;

        trace!("Attempting to load Amidi library from: {}", path);
        let lib = Library::new(path)?;

        macro_rules! load_sym {
            ($sym:literal) => {
                unsafe {
                    lib.get::<$sym>()?
                }
            };
        }

        // Build the final struct with all required symbols loaded.
        let loaded = AmidiLibrary {
            library: lib,

            amidi_device_from_java: load_sym!(b"AMidiDevice_fromJava"),
            amidi_device_release: load_sym!(b"AMidiDevice_release"),
            amidi_device_get_type: load_sym!(b"AMidiDevice_getType"),
            amidi_device_get_num_input_ports: load_sym!(b"AMidiDevice_getNumInputPorts"),
            amidi_device_get_num_output_ports: load_sym!(b"AMidiDevice_getNumOutputPorts"),
            amidi_device_get_default_protocol: load_sym!(b"AMidiDevice_getDefaultProtocol"),

            amidi_output_port_open: load_sym!(b"AMidiOutputPort_open"),
            amidi_output_port_close: load_sym!(b"AMidiOutputPort_close"),
            amidi_output_port_receive: load_sym!(b"AMidiOutputPort_receive"),

            amidi_input_port_open: load_sym!(b"AMidiInputPort_open"),
            amidi_input_port_send: load_sym!(b"AMidiInputPort_send"),
            amidi_input_port_send_with_timestamp: load_sym!(b"AMidiInputPort_sendWithTimestamp"),
            amidi_input_port_send_flush: load_sym!(b"AMidiInputPort_sendFlush"),
            amidi_input_port_close: load_sym!(b"AMidiInputPort_close"),
        };

        info!("Successfully loaded libamidi.so and all symbols.");
        Ok(Arc::new(loaded))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tests for the interface (not the underlying implementation).
// We do not rely on an actual device here, so these tests might be more of a
// "smoke test" pattern, verifying library loading etc.
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test_amidi_library {
    use super::*;

    #[traced_test]
    fn test_library_load_failure() {
        info!("Testing error on invalid path...");
        // Attempt to load a nonsense path. We expect an error.
        let result = AmidiLibrary::builder().path("does_not_exist_libamidi.so").build();
        assert!(result.is_err(), "Should fail to load invalid library path");
    }

    // If you have a real path and environment, you can rename and enable a test like:
    // #[traced_test]
    // fn test_library_load_success() {
    //     let lib = AmidiLibrary::builder()
    //         .path("/path/to/real/libamidi.so")
    //         .build()
    //         .expect("Should load libamidi successfully");
    //     info!("Successfully loaded the library in test: {:?}", lib);
    // }
}
