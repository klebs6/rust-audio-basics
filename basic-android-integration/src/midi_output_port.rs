// ---------------- [ File: basic-android-integration/src/midi_output_port.rs ]
crate::ix!();

#[derive(Debug)]
pub struct MidiOutputPort<'lib> {
    pub(crate) library: Arc<AmidiLibrary>,
    pub(crate) raw_out: *mut AMidiOutputPort,
    pub(crate) _marker: std::marker::PhantomData<&'lib AmidiLibrary>,
}

impl<'lib> MidiOutputPort<'lib> {
    /// Receive MIDI data (up to `buffer.len()` bytes).
    /// Returns `Ok(())` if successful, or `Err(isize)` if < 0 from the C API.
    pub fn receive(
        &self,
        opcode_ptr: &mut i32,
        buffer: &mut [u8],
        num_bytes_received: &mut usize,
        timestamp: &mut i64,
    ) -> Result<(), isize> {
        trace!("Receiving MIDI data on output port...");
        let ret = unsafe {
            (self.library.amidi_output_port_receive)(
                self.raw_out,
                opcode_ptr,
                buffer.as_mut_ptr(),
                buffer.len(),
                num_bytes_received,
                timestamp,
            )
        };
        if ret < 0 {
            error!("AMidiOutputPort_receive returned error: {}", ret);
            Err(ret)
        } else {
            debug!(
                "Received {} bytes, opcode={}, timestamp={}",
                *num_bytes_received, *opcode_ptr, *timestamp
            );
            Ok(())
        }
    }
}

impl<'lib> Drop for MidiOutputPort<'lib> {
    fn drop(&mut self) {
        if !self.raw_out.is_null() {
            trace!("Dropping MidiOutputPort => closing...");
            unsafe {
                (self.library.amidi_output_port_close)(self.raw_out);
            }
            self.raw_out = std::ptr::null_mut();
        }
    }
}
