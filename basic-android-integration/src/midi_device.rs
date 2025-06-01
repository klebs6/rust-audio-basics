// ---------------- [ File: basic-android-integration/src/midi_device.rs ]
crate::ix!();

#[derive(Debug)]
pub struct MidiDevice<'lib> {
    library: Arc<AmidiLibrary>,
    raw_device: *mut AMidiDevice,
    /// We store a lifetime reference ensuring the `AmidiLibrary`
    /// outlives this `MidiDevice`.
    _marker: std::marker::PhantomData<&'lib AmidiLibrary>,
}

impl<'lib> MidiDevice<'lib> {
    /// Create a new `MidiDevice` from an existing Java MIDI device object.
    /// The caller must ensure the `JNIEnv` and `jobject` remain valid for the call.
    pub unsafe fn from_java(
        library: Arc<AmidiLibrary>,
        env: *mut JNIEnv,
        midi_device_obj: jobject,
    ) -> Result<MidiDevice<'lib>, i32> {
        trace!("Creating MidiDevice fromJava...");

        let mut raw_dev: *mut AMidiDevice = std::ptr::null_mut();
        let status = (library.amidi_device_from_java)(env, midi_device_obj, &mut raw_dev);
        if status != 0 {
            error!("AMidiDevice_fromJava failed with status: {}", status);
            return Err(status);
        }

        debug!("MidiDevice pointer (fromJava) = {:p}", raw_dev);
        Ok(MidiDevice {
            library,
            raw_device: raw_dev,
            _marker: std::marker::PhantomData,
        })
    }

    /// Returns the device's type (1 = USB, 2 = VIRTUAL, 3 = BLUETOOTH, etc.).
    /// See your FFI constants for specifics.
    pub fn device_type(&self) -> i32 {
        trace!("Getting device type...");
        unsafe { (self.library.amidi_device_get_type)(self.raw_device) }
    }

    /// Return the number of input ports on this MIDI device.
    pub fn num_input_ports(&self) -> isize {
        trace!("Getting number of input ports...");
        unsafe { (self.library.amidi_device_get_num_input_ports)(self.raw_device) }
    }

    /// Return the number of output ports on this MIDI device.
    pub fn num_output_ports(&self) -> isize {
        trace!("Getting number of output ports...");
        unsafe { (self.library.amidi_device_get_num_output_ports)(self.raw_device) }
    }

    /// Return the default protocol for this device (MIDI 1.0, MIDI 2.0, etc.).
    /// The raw value is from an `AMidiDevice_Protocol`.
    pub fn default_protocol(&self) -> AMidiDevice_Protocol {
        trace!("Getting default protocol...");
        unsafe { (self.library.amidi_device_get_default_protocol)(self.raw_device) }
    }

    /// Open the specified output port index, returning a `MidiOutputPort` handle.
    pub fn open_output_port(
        &self,
        port_number: i32,
    ) -> Result<MidiOutputPort<'lib>, i32> {
        trace!("Opening MIDI output port #{}...", port_number);
        let mut raw_out: *mut AMidiOutputPort = std::ptr::null_mut();
        let status = unsafe {
            (self.library.amidi_output_port_open)(self.raw_device, port_number, &mut raw_out)
        };
        if status != 0 {
            error!("AMidiOutputPort_open failed with status: {}", status);
            return Err(status);
        }
        debug!("MidiOutputPort pointer = {:p}", raw_out);
        Ok(MidiOutputPort {
            library: self.library.clone(),
            raw_out,
            _marker: std::marker::PhantomData,
        })
    }

    /// Open the specified input port index, returning a `MidiInputPort` handle.
    pub fn open_input_port(
        &self,
        port_number: i32,
    ) -> Result<MidiInputPort<'lib>, i32> {
        trace!("Opening MIDI input port #{}...", port_number);
        let mut raw_in: *mut AMidiInputPort = std::ptr::null_mut();
        let status = unsafe {
            (self.library.amidi_input_port_open)(self.raw_device, port_number, &mut raw_in)
        };
        if status != 0 {
            error!("AMidiInputPort_open failed with status: {}", status);
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
    /// RAII cleanup. Releases the underlying `AMidiDevice`.
    fn drop(&mut self) {
        if !self.raw_device.is_null() {
            trace!("Dropping MidiDevice => releasing AMidiDevice...");
            let status =
                unsafe { (self.library.amidi_device_release)(self.raw_device) };
            if status != 0 {
                warn!("AMidiDevice_release returned error status: {}", status);
            }
            self.raw_device = std::ptr::null_mut();
        }
    }
}
