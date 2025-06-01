crate::ix!();

///////////////////////////////////////////////////////////////////////////////
// Safe RAII wrapper for a MIDI output port
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct MidiOutputPort<'lib> {
    library: Arc<AmidiLibrary>,
    raw_out: *mut AMidiOutputPort,
    _marker: std::marker::PhantomData<&'lib AmidiLibrary>,
}

impl<'lib> MidiOutputPort<'lib> {
    /// Receives a MIDI message (up to `buffer.len()` bytes).
    /// - `opcode_ptr` receives the opcode (AMIDI_OPCODE_DATA, etc.).
    /// - `num_bytes_received` will contain actual bytes read.
    /// - `timestamp` will contain the event's timestamp, if available.
    ///
    /// Returns `Ok(())` if successful, or `Err(isize)` if the function call fails
    /// (negative value or other).
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
                opcode_ptr as *mut i32,
                buffer.as_mut_ptr(),
                buffer.len(),
                num_bytes_received as *mut usize,
                timestamp as *mut i64,
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
