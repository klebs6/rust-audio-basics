// ---------------- [ File: basic-android-integration/src/midi_input_port.rs ]
crate::ix!();

#[derive(Debug)]
pub struct MidiInputPort<'lib> {
    pub(crate) library: Arc<AmidiLibrary>,
    pub(crate) raw_in: *mut AMidiInputPort,
    pub(crate) _marker: std::marker::PhantomData<&'lib AmidiLibrary>,
}

impl<'lib> MidiInputPort<'lib> {
    /// Send MIDI data immediately. Returns number of bytes sent if >= 0,
    /// or `Err(isize)` on failure (< 0).
    pub fn send(&self, buffer: &[u8]) -> Result<usize, isize> {
        trace!("Sending MIDI data on input port (no timestamp)...");
        let ret = unsafe {
            (self.library.amidi_input_port_send)(
                self.raw_in,
                buffer.as_ptr(),
                buffer.len(),
            )
        };
        if ret < 0 {
            error!("AMidiInputPort_send error: {}", ret);
            Err(ret)
        } else {
            debug!("Sent {} bytes (no timestamp).", ret);
            Ok(ret as usize)
        }
    }

    /// Send MIDI data with a specific timestamp. Returns number of bytes sent
    /// if >= 0, or `Err(isize)` on failure.
    pub fn send_with_timestamp(&self, buffer: &[u8], timestamp: i64) -> Result<usize, isize> {
        trace!("Sending MIDI data on input port with timestamp={}", timestamp);
        let ret = unsafe {
            (self.library.amidi_input_port_send_with_timestamp)(
                self.raw_in,
                buffer.as_ptr(),
                buffer.len(),
                timestamp,
            )
        };
        if ret < 0 {
            error!("AMidiInputPort_sendWithTimestamp error: {}", ret);
            Err(ret)
        } else {
            debug!("Sent {} bytes with timestamp={}.", ret, timestamp);
            Ok(ret as usize)
        }
    }

    /// Flushes any queued data. Returns `Ok(())` if status==0, otherwise `Err(status)`.
    pub fn send_flush(&self) -> Result<(), media_status_t> {
        trace!("Sending FLUSH to MIDI input port...");
        let status = unsafe {
            (self.library.amidi_input_port_send_flush)(self.raw_in)
        };
        if status != media_status_t(0) {
            error!("AMidiInputPort_sendFlush returned error: {:?}", status);
            Err(status)
        } else {
            debug!("Flushed MIDI input port successfully.");
            Ok(())
        }
    }
}

impl<'lib> Drop for MidiInputPort<'lib> {
    fn drop(&mut self) {
        if !self.raw_in.is_null() {
            trace!("Dropping MidiInputPort => closing...");
            unsafe {
                (self.library.amidi_input_port_close)(self.raw_in);
            }
            self.raw_in = std::ptr::null_mut();
        }
    }
}
