// ---------------- [ File: basic-android-integration/src/midi_device.rs ]
crate::ix!();

#[derive(Debug)]
pub struct MidiDevice<'lib> {
    library: Arc<AmidiLibrary>,
    raw_device: *mut AMidiDevice,
    _marker: std::marker::PhantomData<&'lib AmidiLibrary>,
}

impl<'lib> MidiDevice<'lib> {
    /// Create a `MidiDevice` from an existing Java MIDI device object (JNI).
    /// Returns `media_status_t` if there's an error from the C API.
    ///
    /// # Safety
    /// The `env` and `midi_device_obj` must be valid JNI references.
    pub unsafe fn from_java(
        library: Arc<AmidiLibrary>,
        env: *mut JNIEnv,
        midi_device_obj: jobject,
    ) -> Result<Self, media_status_t> {
        trace!("Creating MidiDevice fromJava...");

        let mut raw_dev: *mut AMidiDevice = std::ptr::null_mut();
        let status = (library.amidi_device_from_java)(env, midi_device_obj, &mut raw_dev);
        if status != media_status_t(0) {
            error!("AMidiDevice_fromJava failed with status: {:?}", status);
            return Err(status);
        }

        debug!("MidiDevice pointer (fromJava) = {:p}", raw_dev);
        Ok(Self {
            library,
            raw_device: raw_dev,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn device_type(&self) -> i32 {
        trace!("Getting device type...");
        unsafe { (self.library.amidi_device_get_type)(self.raw_device) }
    }

    pub fn num_input_ports(&self) -> isize {
        trace!("Getting number of input ports...");
        unsafe { (self.library.amidi_device_get_num_input_ports)(self.raw_device) }
    }

    pub fn num_output_ports(&self) -> isize {
        trace!("Getting number of output ports...");
        unsafe { (self.library.amidi_device_get_num_output_ports)(self.raw_device) }
    }

    pub fn default_protocol(&self) -> AMidiDevice_Protocol {
        trace!("Getting default protocol...");
        unsafe { (self.library.amidi_device_get_default_protocol)(self.raw_device) }
    }

    pub fn open_output_port(
        &self,
        port_number: i32,
    ) -> Result<MidiOutputPort<'lib>, media_status_t> {
        trace!("Opening MIDI output port #{}...", port_number);
        let mut raw_out: *mut AMidiOutputPort = std::ptr::null_mut();
        let status = unsafe {
            (self.library.amidi_output_port_open)(self.raw_device, port_number, &mut raw_out)
        };
        if status != media_status_t(0) {
            error!("AMidiOutputPort_open failed with status: {:?}", status);
            return Err(status);
        }
        debug!("MidiOutputPort pointer = {:p}", raw_out);
        Ok(MidiOutputPort {
            library: self.library.clone(),
            raw_out,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn open_input_port(
        &self,
        port_number: i32,
    ) -> Result<MidiInputPort<'lib>, media_status_t> {
        trace!("Opening MIDI input port #{}...", port_number);
        let mut raw_in: *mut AMidiInputPort = std::ptr::null_mut();
        let status = unsafe {
            (self.library.amidi_input_port_open)(self.raw_device, port_number, &mut raw_in)
        };
        if status != media_status_t(0) {
            error!("AMidiInputPort_open failed with status: {:?}", status);
            return Err(status);
        }
        debug!("MidiInputPort pointer = {:p}", raw_in);
        Ok(MidiInputPort {
            library: self.library.clone(),
            raw_in,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<'lib> Drop for MidiDevice<'lib> {
    fn drop(&mut self) {
        if !self.raw_device.is_null() {
            trace!("Dropping MidiDevice => releasing AMidiDevice...");
            let status = unsafe {
                (self.library.amidi_device_release)(self.raw_device)
            };
            if status != media_status_t(0) {
                warn!("AMidiDevice_release returned error status: {:?}", status);
            }
            self.raw_device = std::ptr::null_mut();
        }
    }
}
